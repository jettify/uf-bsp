use core::num::NonZeroU32;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// Error returned when attempting to claim an [`IrqLatchState`] that already has an active [`IrqLatch`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IrqLatchClaimError;

/// Shared IRQ-backed state for a single-consumer counted async latch.
pub struct IrqLatchState {
    count: AtomicU32,
    claimed: AtomicBool,
    signal: Signal<CriticalSectionRawMutex, u32>,
}

impl IrqLatchState {
    pub const fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
            claimed: AtomicBool::new(false),
            signal: Signal::new(),
        }
    }

    pub fn current_count(&self) -> u32 {
        self.count.load(Ordering::Acquire)
    }

    pub fn on_irq(&self) {
        let count = self.count.fetch_add(1, Ordering::AcqRel).wrapping_add(1);
        self.signal.signal(count);
    }

    fn try_claim(&self) -> Result<(), IrqLatchClaimError> {
        self.claimed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map(drop)
            .map_err(|_already_claimed| IrqLatchClaimError)
    }

    fn release(&self) {
        self.claimed.store(false, Ordering::Release);
    }

    async fn wait_for_change(&self, last_seen: u32) -> u32 {
        let mut last_seen = last_seen;

        loop {
            let count = self.current_count();
            if count != last_seen {
                return count;
            }

            let signaled = self.signal.wait().await;
            if signaled != last_seen {
                return signaled;
            }

            last_seen = signaled;
        }
    }
}

impl Default for IrqLatchState {
    fn default() -> Self {
        Self::new()
    }
}

/// Owns the hardware input that keeps the IRQ line configured and exposes exclusive single-consumer
/// awaiting over an [`IrqLatchState`].
pub struct IrqLatch<I> {
    _input: I,
    state: &'static IrqLatchState,
    last_seen: u32,
}

impl<I> IrqLatch<I> {
    /// Wrap `input` and attach to `state`.
    ///
    /// Prefer [`IrqLatch::try_new`] when `state` is not already guaranteed to be uniquely owned by a
    /// hardware singleton token.
    pub fn new_unchecked(input: I, state: &'static IrqLatchState) -> Self {
        state.claimed.store(true, Ordering::Release);
        let last_seen = state.current_count();
        Self {
            _input: input,
            state,
            last_seen,
        }
    }

    /// Claim exclusive consumer ownership of `state` and bind the hardware `input` lifetime.
    ///
    /// Returns `Err((input, IrqLatchClaimError))` if `state` is already claimed by another live [`IrqLatch`].
    pub fn try_new(
        input: I,
        state: &'static IrqLatchState,
    ) -> Result<Self, (I, IrqLatchClaimError)> {
        if let Err(err) = state.try_claim() {
            return Err((input, err));
        }

        let last_seen = state.current_count();
        Ok(Self {
            _input: input,
            state,
            last_seen,
        })
    }

    /// Return the current monotonic interrupt count from the underlying state.
    pub fn current_count(&self) -> u32 {
        self.state.current_count()
    }

    /// Return the last interrupt count observed by this latch consumer.
    pub fn last_seen(&self) -> u32 {
        self.last_seen
    }

    /// Wait until at least one new interrupt has fired since the last observation and return the
    /// number of elapsed interrupts (`1` = normal, `> 1` = multiple interrupts arrived before wake).
    pub async fn wait(&mut self) -> NonZeroU32 {
        let current = self.state.wait_for_change(self.last_seen).await;
        let delta = current.wrapping_sub(self.last_seen);
        self.last_seen = current;
        NonZeroU32::new(delta).unwrap_or(NonZeroU32::MIN)
    }
}

impl<I> Drop for IrqLatch<I> {
    fn drop(&mut self) {
        self.state.release();
    }
}

#[cfg(test)]
mod tests {
    use core::future::Future;
    use core::pin::pin;
    use core::sync::atomic::AtomicBool;
    use core::sync::atomic::Ordering;
    use core::task::Context;
    use core::task::Poll;
    use core::task::RawWaker;
    use core::task::RawWakerVTable;
    use core::task::Waker;

    use super::*;

    fn noop_waker() -> Waker {
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(core::ptr::null(), &VTABLE)
        }
        fn wake(_: *const ()) {}
        fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);
        // SAFETY: The vtable functions are stateless no-ops and do not dereference the data pointer.
        unsafe { Waker::from_raw(RawWaker::new(core::ptr::null(), &VTABLE)) }
    }

    fn flag_waker(flag: &'static AtomicBool) -> Waker {
        fn clone(ptr: *const ()) -> RawWaker {
            RawWaker::new(ptr, &VTABLE)
        }
        fn wake(ptr: *const ()) {
            // SAFETY: `ptr` points to a `'static AtomicBool` passed to `flag_waker`.
            let flag = unsafe { &*ptr.cast::<AtomicBool>() };
            flag.store(true, Ordering::SeqCst);
        }
        fn drop(_: *const ()) {}

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake, drop);
        let ptr = core::ptr::from_ref(flag).cast::<()>();
        // SAFETY: `ptr` is valid for `'static` and the vtable only atomically stores `true`.
        unsafe { Waker::from_raw(RawWaker::new(ptr, &VTABLE)) }
    }

    #[test]
    fn counter_increments_and_wraps() {
        let state = IrqLatchState::new();
        assert_eq!(state.current_count(), 0, "initial count must be 0");

        state.on_irq();
        assert_eq!(state.current_count(), 1, "count must increment to 1");

        state.count.store(u32::MAX, Ordering::Release);
        state.on_irq();
        assert_eq!(
            state.current_count(),
            0,
            "count must wrap around u32::MAX to 0"
        );
    }

    #[test]
    fn single_consumer_claim_and_drop_reclaim() {
        static STATE: IrqLatchState = IrqLatchState::new();

        let latch = IrqLatch::try_new(42u8, &STATE);
        assert!(latch.is_ok(), "first claim must succeed");

        let duplicate = IrqLatch::try_new(99u8, &STATE);
        assert_eq!(
            duplicate.err(),
            Some((99u8, IrqLatchClaimError)),
            "second claim while first is alive must fail and return input"
        );

        drop(latch);

        let reclaimed = IrqLatch::try_new(7u8, &STATE);
        assert!(reclaimed.is_ok(), "claim after drop must succeed");
    }

    #[test]
    fn wait_returns_immediately_when_already_signaled() {
        static STATE: IrqLatchState = IrqLatchState::new();

        let Ok(mut latch) = IrqLatch::try_new((), &STATE) else {
            return;
        };

        STATE.on_irq();
        STATE.on_irq();
        STATE.on_irq();

        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        let poll = {
            let mut fut = pin!(latch.wait());
            fut.as_mut().poll(&mut cx)
        };

        assert_eq!(
            poll,
            Poll::Ready(NonZeroU32::new(3).unwrap_or(NonZeroU32::MIN)),
            "wait must immediately return the 3 accumulated pulses"
        );
        assert_eq!(
            latch.last_seen(),
            latch.current_count(),
            "last_seen must catch up"
        );
    }

    #[test]
    fn wait_suspends_and_wakes_on_irq() {
        static STATE: IrqLatchState = IrqLatchState::new();
        static WOKE: AtomicBool = AtomicBool::new(false);

        let Ok(mut latch) = IrqLatch::try_new((), &STATE) else {
            return;
        };

        let waker = flag_waker(&WOKE);
        let mut cx = Context::from_waker(&waker);

        let mut fut = pin!(latch.wait());
        assert_eq!(
            fut.as_mut().poll(&mut cx),
            Poll::Pending,
            "wait must suspend when no IRQ has occurred"
        );
        assert!(
            !WOKE.load(Ordering::SeqCst),
            "waker must not have fired yet"
        );

        STATE.on_irq();
        assert!(
            WOKE.load(Ordering::SeqCst),
            "on_irq must wake the registered task"
        );

        assert_eq!(
            fut.as_mut().poll(&mut cx),
            Poll::Ready(NonZeroU32::MIN),
            "second poll after on_irq must resolve with delta = 1"
        );
    }
}
