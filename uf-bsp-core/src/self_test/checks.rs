use super::CheckOutcome;
use super::SuiteStatus;
use crate::capabilities::PinPolarity;
use embassy_time::Duration;
use embassy_time::Instant;
use embassy_time::Timer;
use embedded_hal::digital::OutputPin;

/// Successful acquisition, not variation, is required: static analog inputs and
/// full-scale 16-bit samples are valid. ADC/DMA setup remains board-specific.
pub fn validate_adc_samples(
    completed_reads: u32,
    samples: &[u16],
    resolution_bits: u8,
) -> CheckOutcome {
    if !(1..=16).contains(&resolution_bits) {
        return CheckOutcome::fail("invalid ADC resolution");
    }
    if completed_reads == 0 || samples.is_empty() {
        return CheckOutcome::fail("no completed ADC acquisitions");
    }
    let max = u16::MAX >> (16 - resolution_bits);
    if samples.iter().any(|sample| *sample > max) {
        return CheckOutcome::fail("ADC sample exceeds configured resolution");
    }
    CheckOutcome::pass("ADC acquisitions completed; raw values, calibration untested")
}

/// Tests time-driver progress only, using its own clock as the reference. A
/// broken time driver can stall this check and its deadline; host observation is required.
pub async fn check_timer() -> CheckOutcome {
    let start = Instant::now();
    Timer::after_millis(20).await;
    if Instant::now() > start {
        CheckOutcome::pass("time driver progressed; absolute frequency untested")
    } else {
        CheckOutcome::fail("monotonic timer did not advance")
    }
}

fn drive_leds<P: OutputPin>(
    leds: &mut [P],
    polarities: &[Option<PinPolarity>],
    active: bool,
) -> Result<(), P::Error> {
    for (led, polarity) in leds.iter_mut().zip(polarities) {
        // Unknown polarity: emit a high pulse with a low idle level, making no
        // claim about optical ON/OFF. Known polarity always idles inactive.
        let high = polarity.map_or(active, |polarity| polarity.level_high(active));
        if high {
            led.set_high()?;
        } else {
            led.set_low()?;
        }
    }
    Ok(())
}

fn set_leds<P: OutputPin>(
    leds: &mut [P],
    polarities: &[Option<PinPolarity>],
    active: bool,
) -> bool {
    if leds.is_empty() || leds.len() != polarities.len() {
        return false;
    }
    drive_leds(leds, polarities, active).is_ok()
}

pub async fn check_leds<P: OutputPin>(
    leds: &mut [P],
    polarities: &[Option<PinPolarity>],
) -> CheckOutcome {
    for _ in 0..3 {
        if !set_leds(leds, polarities, true) {
            return CheckOutcome::fail("LED output configuration/control failed");
        }
        Timer::after_millis(100).await;
        if !set_leds(leds, polarities, false) {
            return CheckOutcome::fail("LED output configuration/control failed");
        }
        Timer::after_millis(100).await;
    }
    CheckOutcome::pass("LED outputs exercised; confirm illumination visually")
}

/// Repeat one pulse for PASS or three for FAIL/INCOMPLETE. Unknown polarity is
/// an electrical high pulse, possibly a dark pulse. GPIO errors are returned.
pub async fn blink_status_leds<P: OutputPin>(
    leds: &mut [P],
    polarities: &[Option<PinPolarity>],
    status: SuiteStatus,
) -> Result<(), P::Error> {
    let pulses = if status == SuiteStatus::Pass { 1 } else { 3 };
    loop {
        for _ in 0..pulses {
            drive_leds(leds, polarities, true)?;
            Timer::after(Duration::from_millis(150)).await;
            drive_leds(leds, polarities, false)?;
            Timer::after(Duration::from_millis(150)).await;
        }
        Timer::after_secs(1).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::self_test::CheckStatus;
    struct Led {
        high: bool,
        broken: bool,
    }
    impl embedded_hal::digital::ErrorType for Led {
        type Error = embedded_hal::digital::ErrorKind;
    }
    impl OutputPin for Led {
        fn set_low(&mut self) -> Result<(), Self::Error> {
            if self.broken {
                return Err(embedded_hal::digital::ErrorKind::Other);
            }
            self.high = false;
            Ok(())
        }
        fn set_high(&mut self) -> Result<(), Self::Error> {
            if self.broken {
                return Err(embedded_hal::digital::ErrorKind::Other);
            }
            self.high = true;
            Ok(())
        }
    }
    #[test]
    fn led_polarity_idle_and_control_errors() {
        let mut leds = [
            Led {
                high: false,
                broken: false,
            },
            Led {
                high: false,
                broken: false,
            },
        ];
        let polarities = [Some(PinPolarity::ActiveLow), None];
        assert!(set_leds(&mut leds, &polarities, true));
        let [active_low, unknown] = &leds;
        assert!(!active_low.high && unknown.high);
        assert!(set_leds(&mut leds, &polarities, false));
        let [active_low, unknown] = &leds;
        assert!(active_low.high && !unknown.high);
        let mut broken = [Led {
            high: false,
            broken: true,
        }];
        assert!(!set_leds(&mut broken, &[None], true));
        assert!(!set_leds(&mut leds, &[None], true));
    }
    #[test]
    fn adc_requires_acquisition_and_accepts_constant_full_scale() {
        assert_eq!(validate_adc_samples(0, &[0], 12).status, CheckStatus::Fail);
        assert_eq!(validate_adc_samples(1, &[], 12).status, CheckStatus::Fail);
        assert_eq!(
            validate_adc_samples(1, &[0, 0], 12).status,
            CheckStatus::Pass
        );
        assert_eq!(
            validate_adc_samples(1, &[u16::MAX; 2], 16).status,
            CheckStatus::Pass
        );
        assert_eq!(
            validate_adc_samples(1, &[4096], 12).status,
            CheckStatus::Fail
        );
        for bits in [0, 17] {
            assert_eq!(
                validate_adc_samples(1, &[0], bits).status,
                CheckStatus::Fail
            );
        }
    }
}
