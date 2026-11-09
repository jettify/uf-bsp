use super::CheckOutcome;
use super::CheckStatus;
use super::Report;
use embedded_hal::i2c::Error as _;
use embedded_hal::i2c::ErrorKind;
use embedded_hal::i2c::I2c;
use embedded_hal_async::spi::SpiDevice;

/// Device identity knowledge supplied by a board, never inferred by core.
#[derive(Clone, Copy)]
pub struct ChipId {
    pub value: u8,
    pub mask: u8,
    pub name: &'static str,
}
impl ChipId {
    fn matches(self, value: u8) -> bool {
        value & self.mask == self.value & self.mask
    }
}

#[derive(Clone, Copy)]
pub struct I2cChipId {
    pub address: u8,
    pub register: u8,
    pub chip: ChipId,
}

/// One command byte followed by one response byte under a single CS assertion.
/// No device configuration is changed. Optional slots returning 0/255 are treated
/// as absent; this cannot distinguish an empty slot from a disconnected device.
pub async fn probe_spi_chip_id<S: SpiDevice, const N: usize>(
    device: &mut S,
    read_command: u8,
    expected: &[ChipId],
    optional: bool,
    report: &mut Report<N>,
) -> CheckOutcome {
    if expected.is_empty() || expected.iter().any(|id| id.mask == 0) {
        return CheckOutcome::fail("invalid SPI identity descriptors");
    }
    let mut bytes = [read_command, 0xff];
    if device.transfer_in_place(&mut bytes).await.is_err() {
        return CheckOutcome::fail("SPI identity transfer failed");
    }
    let [_, value] = bytes;
    report.note(format_args!(
        "SPI command=0x{:02x} identity=0x{:02x}",
        read_command, value
    ));
    if let Some(chip) = expected.iter().find(|chip| chip.matches(value)) {
        report.note(format_args!("identified {}", chip.name));
        return CheckOutcome::pass("SPI identity matched; sensor operation untested");
    }
    if value == 0 || value == 0xff {
        return CheckOutcome::new(
            if optional {
                CheckStatus::Skip
            } else {
                CheckStatus::Fail
            },
            "no SPI identity response",
        );
    }
    CheckOutcome::unsupported("unrecognized SPI identity")
}

/// Probe only the supplied register/address candidates, accepting the first match.
/// Mismatches never stop the search. Blocking I2C must have a controller timeout.
pub fn probe_i2c_chip_id<I: I2c, const N: usize>(
    bus: &mut I,
    candidates: &[I2cChipId],
    optional: bool,
    report: &mut Report<N>,
) -> CheckOutcome {
    if candidates.is_empty()
        || candidates
            .iter()
            .any(|c| c.address > 0x7f || c.chip.mask == 0)
    {
        return CheckOutcome::fail("invalid I2C identity descriptors");
    }
    let mut responded = false;
    let mut bus_error = false;
    for candidate in candidates {
        let mut bytes = [0];
        match bus.write_read(candidate.address, &[candidate.register], &mut bytes) {
            Ok(()) => {
                responded = true;
                let [value] = bytes;
                report.note(format_args!(
                    "I2C address=0x{:02x} register=0x{:02x} identity=0x{:02x}",
                    candidate.address, candidate.register, value
                ));
                if candidate.chip.matches(value) {
                    report.note(format_args!("identified {}", candidate.chip.name));
                    return CheckOutcome::pass("I2C identity matched; sensor operation untested");
                }
            }
            Err(error) => {
                if !matches!(error.kind(), ErrorKind::NoAcknowledge(_)) {
                    bus_error = true;
                }
            }
        }
    }
    if bus_error {
        CheckOutcome::fail("I2C bus error while probing identities")
    } else if responded {
        CheckOutcome::unsupported("unrecognized I2C identities at all candidates")
    } else {
        CheckOutcome::new(
            if optional {
                CheckStatus::Skip
            } else {
                CheckStatus::Fail
            },
            "no I2C identity response",
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::future::Future;
    use core::task::Context;
    use core::task::Poll;
    use core::task::Waker;
    use embedded_hal::i2c::ErrorType;
    use embedded_hal::i2c::NoAcknowledgeSource;
    use embedded_hal::i2c::Operation;
    const ID: ChipId = ChipId {
        value: 0x10,
        mask: 0xf0,
        name: "test chip",
    };
    const CANDIDATES: [I2cChipId; 2] = [
        I2cChipId {
            address: 0x77,
            register: 0x0d,
            chip: ID,
        },
        I2cChipId {
            address: 0x76,
            register: 0x0d,
            chip: ID,
        },
    ];
    struct Bus {
        first: Result<u8, ErrorKind>,
        second: Result<u8, ErrorKind>,
        calls: u8,
    }
    impl ErrorType for Bus {
        type Error = ErrorKind;
    }
    impl I2c for Bus {
        fn transaction(
            &mut self,
            address: u8,
            operations: &mut [Operation<'_>],
        ) -> Result<(), Self::Error> {
            self.calls += 1;
            let value = if address == 0x77 {
                self.first?
            } else {
                self.second?
            };
            for operation in operations {
                if let Operation::Read(bytes) = operation {
                    bytes.fill(value);
                }
            }
            Ok(())
        }
    }
    #[test]
    fn mismatching_first_address_does_not_hide_supported_second() {
        let mut bus = Bus {
            first: Ok(0x50),
            second: Ok(0x12),
            calls: 0,
        };
        assert_eq!(
            probe_i2c_chip_id(&mut bus, &CANDIDATES, false, &mut Report::<512>::new()).status,
            CheckStatus::Pass
        );
        assert_eq!(bus.calls, 2);
    }
    #[test]
    fn i2c_missing_unknown_and_bus_errors_are_distinct() {
        let nack = Err(ErrorKind::NoAcknowledge(NoAcknowledgeSource::Address));
        let mut bus = Bus {
            first: nack,
            second: nack,
            calls: 0,
        };
        let mut report = Report::<512>::new();
        assert_eq!(
            probe_i2c_chip_id(&mut bus, &CANDIDATES, true, &mut report).status,
            CheckStatus::Skip
        );
        assert_eq!(
            probe_i2c_chip_id(&mut bus, &CANDIDATES, false, &mut report).status,
            CheckStatus::Fail
        );
        bus.first = Ok(0x50);
        assert_eq!(
            probe_i2c_chip_id(&mut bus, &CANDIDATES, false, &mut report).status,
            CheckStatus::Unsupported
        );
        bus.second = Err(ErrorKind::Bus);
        assert_eq!(
            probe_i2c_chip_id(&mut bus, &CANDIDATES, true, &mut report).status,
            CheckStatus::Fail
        );
    }
    struct Spi {
        value: u8,
        error: bool,
    }
    impl embedded_hal::spi::ErrorType for Spi {
        type Error = embedded_hal::spi::ErrorKind;
    }
    impl SpiDevice for Spi {
        async fn transaction(
            &mut self,
            operations: &mut [embedded_hal::spi::Operation<'_, u8>],
        ) -> Result<(), Self::Error> {
            if self.error {
                return Err(embedded_hal::spi::ErrorKind::Other);
            }
            for operation in operations {
                if let embedded_hal::spi::Operation::TransferInPlace([command, value]) = operation {
                    assert_eq!(*command, 0xf5);
                    *value = self.value;
                }
            }
            Ok(())
        }
    }
    #[test]
    fn spi_checks_identity_presence_and_transfer_error() {
        for (value, optional, error, expected) in [
            (0x12, false, false, CheckStatus::Pass),
            (0, true, false, CheckStatus::Skip),
            (255, false, false, CheckStatus::Fail),
            (0x50, false, false, CheckStatus::Unsupported),
            (0x12, true, true, CheckStatus::Fail),
        ] {
            let mut report = Report::<256>::new();
            let mut spi = Spi { value, error };
            let mut future = core::pin::pin!(probe_spi_chip_id(
                &mut spi,
                0xf5,
                &[ID],
                optional,
                &mut report
            ));
            assert!(
                matches!(future.as_mut().poll(&mut Context::from_waker(Waker::noop())),
                Poll::Ready(outcome) if outcome.status == expected)
            );
        }
    }
}
