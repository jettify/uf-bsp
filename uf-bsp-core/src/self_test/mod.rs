//! Opt-in, driver-free hardware checks. Peripheral construction and chip identities
//! belong to the BSP. Register checks verify communication, not sensor health.
mod checks;
mod probes;
mod report;
mod usb;

pub use checks::blink_status_leds;
pub use checks::check_leds;
pub use checks::check_timer;
pub use checks::validate_adc_samples;
pub use probes::ChipId;
pub use probes::I2cChipId;
pub use probes::probe_i2c_chip_id;
pub use probes::probe_spi_chip_id;
pub use report::CheckOutcome;
pub use report::CheckStatus;
pub use report::Report;
pub use report::SuiteStatus;
pub use report::Summary;
pub use usb::serve_usb;
