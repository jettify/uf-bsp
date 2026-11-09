use super::Report;
use crate::usb_cdc::CdcAcm;
use core::fmt::Write as _;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::with_timeout;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::driver::Driver;
use embassy_usb::driver::EndpointError;

// A small internal seam permits transport tests without emulating a USB controller.
trait PacketWriter {
    fn packet_size(&self) -> usize;
    async fn write(&mut self, bytes: &[u8]) -> Result<(), EndpointError>;
}
impl<'d, D: Driver<'d>> PacketWriter for CdcAcmClass<'d, D> {
    fn packet_size(&self) -> usize {
        usize::from(self.max_packet_size())
    }
    async fn write(&mut self, bytes: &[u8]) -> Result<(), EndpointError> {
        self.write_packet(bytes).await
    }
}
async fn send<W: PacketWriter>(writer: &mut W, bytes: &[u8]) -> bool {
    for packet in bytes.chunks(writer.packet_size().max(1)) {
        if !matches!(
            with_timeout(Duration::from_millis(250), writer.write(packet)).await,
            Ok(Ok(()))
        ) {
            return false;
        }
    }
    true
}
async fn send_report<W: PacketWriter, const N: usize>(writer: &mut W, report: &Report<N>) -> bool {
    if !send(
        writer,
        b"BEGIN SELF_CHECK\r\nPASS USB: CDC endpoint enabled\r\n",
    )
    .await
    {
        return false;
    }
    if report.overflow() && !send(writer, b"FAIL report buffer overflow; summary invalid\r\n").await
    {
        return false;
    }
    if !send(writer, report.text().as_bytes()).await {
        return false;
    }
    let mut summary = heapless::String::<128>::new();
    let s = report.summary();
    let result = write!(
        summary,
        "SUMMARY {} pass={} fail={} skip={} unsupported={}\r\n",
        report.status().label(),
        s.passed,
        s.failed,
        s.skipped,
        s.unsupported
    );
    if result.is_err()
        || !send(writer, summary.as_bytes()).await
        || !send(writer, b"END SELF_CHECK\r\n").await
    {
        return false;
    }
    // Terminate the final full-size packet, if necessary, so host reads complete.
    if b"END SELF_CHECK\r\n"
        .len()
        .is_multiple_of(writer.packet_size().max(1))
    {
        return matches!(
            with_timeout(Duration::from_millis(250), writer.write(&[])).await,
            Ok(Ok(()))
        );
    }
    true
}
async fn writer<'d, D: Driver<'d>, const N: usize>(
    mut class: CdcAcmClass<'d, D>,
    report: &Report<N>,
) {
    loop {
        // Endpoint enablement is not a DTR/terminal-open indication.
        class.wait_connection().await;
        let sent = send_report(&mut class, report).await;
        // Timeouts do not necessarily disable endpoints. Always back off before retrying.
        Timer::after_millis(if sent { 2000 } else { 500 }).await;
    }
}

/// Run USB and repeat the completed report every two seconds.
pub async fn serve_usb<'d, D: Driver<'d>, const N: usize>(cdc: CdcAcm<'d, D>, report: &Report<N>) {
    let CdcAcm { mut device, class } = cdc;
    embassy_futures::join::join(device.run(), writer(class, report)).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::self_test::CheckOutcome;
    use crate::self_test::CheckStatus;
    use crate::self_test::SuiteStatus;
    use core::future::Future;
    use core::task::Context;
    use core::task::Poll;
    use core::task::Waker;
    struct Writer {
        output: heapless::Vec<u8, 1024>,
        stalled: bool,
    }
    impl PacketWriter for Writer {
        fn packet_size(&self) -> usize {
            8
        }
        async fn write(&mut self, bytes: &[u8]) -> Result<(), EndpointError> {
            if self.stalled {
                core::future::pending::<()>().await;
            }
            assert!(bytes.len() <= 8);
            self.output
                .extend_from_slice(bytes)
                .map_err(|_full| EndpointError::BufferOverflow)
        }
    }
    #[test]
    fn framed_reports_work_with_small_packets() {
        let mut report = Report::<256>::new();
        report.record("chip", CheckOutcome::pass("matched"));
        let mut writer = Writer {
            output: heapless::Vec::new(),
            stalled: false,
        };
        {
            let mut future = core::pin::pin!(send_report(&mut writer, &report));
            assert_eq!(
                future
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(true)
            );
        }
        let output = core::str::from_utf8(&writer.output).unwrap_or("");
        assert!(output.starts_with("BEGIN SELF_CHECK\r\n"));
        assert!(output.contains("SUMMARY PASS pass=1"));
        assert!(output.ends_with("END SELF_CHECK\r\n"));
    }
    // Time-based tests share the single mock-driver clock and run in one test.
    #[test]
    fn deadlines_and_stalled_usb_are_bounded_and_retryable() {
        let clock = embassy_time::MockDriver::get();
        let mut cx = Context::from_waker(Waker::noop());
        let mut writer = Writer {
            output: heapless::Vec::new(),
            stalled: true,
        };
        {
            let mut future = core::pin::pin!(send(&mut writer, b"packet"));
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Pending);
            clock.advance(Duration::from_millis(251));
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(false));
        }
        writer.stalled = false;
        {
            let mut future = core::pin::pin!(send(&mut writer, b"packet"));
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(true));
        }
        let mut report = Report::<256>::new();
        {
            let mut future = core::pin::pin!(
                report.run_check("stalled", async |_| core::future::pending().await)
            );
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Pending);
            clock.advance(Duration::from_secs(6));
            assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(()));
        }
        assert_eq!(report.status(), SuiteStatus::Fail);
        {
            let mut future = core::pin::pin!(crate::self_test::check_timer());
            assert!(matches!(future.as_mut().poll(&mut cx), Poll::Pending));
            clock.advance(Duration::from_millis(21));
            assert!(
                matches!(future.as_mut().poll(&mut cx), Poll::Ready(result) if result.status == CheckStatus::Pass)
            );
        }
    }
}
