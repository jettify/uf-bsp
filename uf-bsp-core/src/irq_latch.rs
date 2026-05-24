use core::sync::atomic::AtomicU32;
use core::sync::atomic::Ordering;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;

/// Shared IRQ-backed state for a counted async latch.
pub struct IrqLatchState {
    count: AtomicU32,
    signal: Signal<CriticalSectionRawMutex, u32>,
}

impl IrqLatchState {
    pub const fn new() -> Self {
        Self {
            count: AtomicU32::new(0),
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

/// Owns the hardware input that keeps the IRQ line configured and exposes the latch state.
pub struct IrqLatch<I> {
    _input: I,
    state: &'static IrqLatchState,
}

impl<I> IrqLatch<I> {
    pub fn new(input: I, state: &'static IrqLatchState) -> Self {
        Self {
            _input: input,
            state,
        }
    }

    pub fn current_count(&self) -> u32 {
        self.state.current_count()
    }

    pub async fn wait(&self, last_seen: &mut u32) {
        *last_seen = self.state.wait_for_change(*last_seen).await;
    }
}
