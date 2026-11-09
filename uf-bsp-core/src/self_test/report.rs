use core::fmt::Write as _;
use embassy_time::Duration;
use embassy_time::with_timeout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckStatus {
    Pass,
    Fail,
    Skip,
    Unsupported,
}
impl CheckStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Skip => "SKIP",
            Self::Unsupported => "UNSUPPORTED",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckOutcome {
    pub status: CheckStatus,
    pub detail: &'static str,
}
impl CheckOutcome {
    pub const fn new(status: CheckStatus, detail: &'static str) -> Self {
        Self { status, detail }
    }
    pub const fn pass(detail: &'static str) -> Self {
        Self::new(CheckStatus::Pass, detail)
    }
    pub const fn fail(detail: &'static str) -> Self {
        Self::new(CheckStatus::Fail, detail)
    }
    pub const fn skip(detail: &'static str) -> Self {
        Self::new(CheckStatus::Skip, detail)
    }
    pub const fn unsupported(detail: &'static str) -> Self {
        Self::new(CheckStatus::Unsupported, detail)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Summary {
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub unsupported: u32,
}
impl Summary {
    fn add(&mut self, status: CheckStatus) {
        let counter = match status {
            CheckStatus::Pass => &mut self.passed,
            CheckStatus::Fail => &mut self.failed,
            CheckStatus::Skip => &mut self.skipped,
            CheckStatus::Unsupported => &mut self.unsupported,
        };
        *counter = counter.saturating_add(1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuiteStatus {
    Pass,
    Fail,
    Incomplete,
}
impl SuiteStatus {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pass => "PASS",
            Self::Fail => "FAIL",
            Self::Incomplete => "INCOMPLETE",
        }
    }
}

/// Bounded, single-owner diagnostic report with no allocator, mutex, or global state.
/// Populate sequentially via `&mut Report<N>`, then share `&Report<N>` with USB serving.
pub struct Report<const N: usize> {
    text: heapless::String<N>,
    summary: Summary,
    overflow: bool,
}
impl<const N: usize> Default for Report<N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<const N: usize> Report<N> {
    pub const fn new() -> Self {
        Self {
            text: heapless::String::new(),
            summary: Summary {
                passed: 0,
                failed: 0,
                skipped: 0,
                unsupported: 0,
            },
            overflow: false,
        }
    }
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
    pub const fn summary(&self) -> Summary {
        self.summary
    }
    pub const fn overflow(&self) -> bool {
        self.overflow
    }
    pub const fn status(&self) -> SuiteStatus {
        if self.overflow || self.summary.failed != 0 {
            SuiteStatus::Fail
        } else if self.summary.unsupported != 0 || self.summary.passed == 0 {
            SuiteStatus::Incomplete
        } else {
            SuiteStatus::Pass
        }
    }
    fn append(&mut self, args: core::fmt::Arguments<'_>) {
        if self.text.write_fmt(args).is_err() || self.text.write_str("\r\n").is_err() {
            self.overflow = true;
        }
    }
    pub fn note(&mut self, args: core::fmt::Arguments<'_>) {
        self.append(args);
    }
    pub fn record(&mut self, name: &str, outcome: CheckOutcome) {
        self.summary.add(outcome.status);
        self.append(format_args!(
            "{} {}: {}",
            outcome.status.label(),
            name,
            outcome.detail
        ));
    }
    pub fn run_sync_check(&mut self, name: &str, check: impl FnOnce(&mut Self) -> CheckOutcome) {
        self.note(format_args!("RUNNING {}", name));
        let outcome = check(self);
        self.record(name, outcome);
    }
    pub async fn run_check<F>(&mut self, name: &str, check: F)
    where
        F: AsyncFnOnce(&mut Self) -> CheckOutcome,
    {
        self.run_check_with_timeout(name, Duration::from_secs(5), check)
            .await;
    }
    pub async fn run_check_with_timeout<F>(&mut self, name: &str, timeout: Duration, check: F)
    where
        F: AsyncFnOnce(&mut Self) -> CheckOutcome,
    {
        self.note(format_args!("RUNNING {}", name));
        let outcome = match with_timeout(timeout, check(self)).await {
            Ok(outcome) => outcome,
            Err(_) => CheckOutcome::fail("check deadline exceeded"),
        };
        self.record(name, outcome);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn lifecycle_and_optional_skips() {
        let mut report = Report::<256>::new();
        report.record("optional", CheckOutcome::skip("absent"));
        assert_eq!(report.status(), SuiteStatus::Incomplete);

        let mut report = Report::<256>::new();
        report.record("required", CheckOutcome::pass("ok"));
        report.record("optional", CheckOutcome::skip("absent"));
        assert_eq!(report.status(), SuiteStatus::Pass);
        assert_eq!(report.summary().passed, 1);
        assert_eq!(report.summary().skipped, 1);
    }
    #[test]
    fn unsupported_and_failures() {
        let mut report = Report::<256>::new();
        report.record("chip", CheckOutcome::unsupported("unknown"));
        assert_eq!(report.status(), SuiteStatus::Incomplete);

        report.record("bus", CheckOutcome::fail("timeout"));
        assert_eq!(report.status(), SuiteStatus::Fail);
    }
    #[test]
    fn overflow_never_passes_and_empty_is_incomplete() {
        let mut report = Report::<8>::new();
        report.record("chip", CheckOutcome::pass("ok"));
        assert!(report.overflow());
        assert_eq!(report.status(), SuiteStatus::Fail);
        assert_eq!(Report::<8>::new().status(), SuiteStatus::Incomplete);
    }
    #[test]
    fn sync_check_records_running_and_outcome_in_order() {
        let mut report = Report::<256>::new();
        report.run_sync_check("baro", |r| {
            r.note(format_args!("probed 0x77"));
            CheckOutcome::pass("matched")
        });
        assert_eq!(
            report.text(),
            "RUNNING baro\r\nprobed 0x77\r\nPASS baro: matched\r\n"
        );
        assert_eq!(report.status(), SuiteStatus::Pass);
    }
}
