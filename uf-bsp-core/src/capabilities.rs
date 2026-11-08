//! Hardware metadata and ownership-preserving interfaces shared by BSPs.
//!
//! These types contain no HAL dependency. Resource type parameters can be owned
//! peripheral tokens (including their lifetimes), drivers, or test doubles.

/// One full-duplex UART and its wired pins. DMA allocation remains board-specific.
pub struct UartPort<U, Tx, Rx> {
    pub uart: U,
    pub tx: Tx,
    pub rx: Rx,
}

/// A UART for which the board routes only RX.
pub struct RxOnlyUartPort<U, Rx> {
    pub uart: U,
    pub rx: Rx,
}

/// Mounting alignment, using the names and convention of the target headers.
/// This describes the sensor mounting; drivers do not apply it automatically.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImuAlignment {
    Cw90Deg,
    Cw90DegFlip,
    Cw0DegFlip,
}

/// Electrical level that activates an output, independent of GPIO configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinPolarity {
    ActiveHigh,
    ActiveLow,
}

impl PinPolarity {
    /// Required electrical level (`true` = high) for the requested logical state.
    pub const fn level_high(self, active: bool) -> bool {
        match self {
            Self::ActiveHigh => active,
            Self::ActiveLow => !active,
        }
    }
}

/// Raw Betaflight meter settings from the board target header.
/// These are calibration defaults, not multipliers for raw ADC samples. ADC
/// resolution, reference voltage, offsets and calibration must be handled by the
/// application. `None` means the header does not specify a board default.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AnalogScale {
    /// `DEFAULT_VOLTAGE_METER_SCALE`, if specified.
    pub voltage_meter_scale: Option<u16>,
    /// `DEFAULT_CURRENT_METER_SCALE`, if specified.
    pub current_meter_scale: Option<u16>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageBus {
    Sdmmc { data_lines: u8 },
    Spi,
}

/// Common storage discovery without erasing bus-specific peripheral types.
/// Applications select a driver appropriate for `BUS`; this does not initialize it.
pub trait StorageCapability {
    const BUS: StorageBus;
}

pub trait ImuMetadata {
    const ALIGNMENT: ImuAlignment;
}

/// Consuming constructor for a board's barometer bus, usable in generic bring-up.
pub trait BlockingI2c: Sized {
    type Config;
    type Bus;

    fn new_i2c_blocking(self, config: Self::Config) -> Self::Bus;
}

/// Common board discovery. Borrowing accessors preserve every resource token;
/// public board fields remain available for moving parts into drivers.
pub trait BoardCapabilities {
    const NAME: &'static str;
    type PrimaryImu: ImuMetadata;
    type Storage: StorageCapability;

    fn primary_imu(&self) -> &Self::PrimaryImu;
    fn storage(&self) -> &Self::Storage;
}

#[cfg(test)]
mod tests {
    use super::PinPolarity;

    #[test]
    fn polarity_maps_both_logical_states() {
        assert!(PinPolarity::ActiveHigh.level_high(true), "active high on");
        assert!(
            !PinPolarity::ActiveHigh.level_high(false),
            "active high off"
        );
        assert!(!PinPolarity::ActiveLow.level_high(true), "active low on");
        assert!(PinPolarity::ActiveLow.level_high(false), "active low off");
    }
}
