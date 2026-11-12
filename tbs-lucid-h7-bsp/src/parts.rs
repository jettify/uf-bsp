use crate::core::capabilities::AnalogScale;
use crate::core::capabilities::BlockingI2c;
use crate::core::capabilities::ImuAlignment;
use crate::core::capabilities::ImuMetadata;
use crate::core::capabilities::PinPolarity;
use crate::core::capabilities::StorageBus;
use crate::core::capabilities::StorageCapability;
use crate::core::capabilities::UartPort;
use crate::hal;
use crate::hal::interrupt::InterruptExt as _;
use crate::pins;

/// Standard Embassy async EXTI input for an IMU data-ready line.
///
/// Waits are armed when polled, and cancellation masks only the owned line.
/// Events between waits are not counted; obtain sample counts from the IMU FIFO.
pub type ImuDrdyInput<'d> = hal::exti::ExtiInput<'d, hal::mode::Async>;

/// Embassy handler for the primary IMU data-ready line (`PB2` / `EXTI2`).
///
/// Bind the vector once, including when other inputs share it. The same binding
/// also supports standard Embassy EXTI inputs on the other lines of that vector.
pub type PrimaryImuDrdyInterruptHandler =
    hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI2>;

pub trait PrimaryImuDrdyIrqs:
    hal::interrupt::typelevel::Binding<hal::interrupt::typelevel::EXTI2, PrimaryImuDrdyInterruptHandler>
{
}

impl<T> PrimaryImuDrdyIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            hal::interrupt::typelevel::EXTI2,
            PrimaryImuDrdyInterruptHandler,
        >
{
}

/// Construct the primary IMU async data-ready input (`PB2` / `EXTI2`).
///
/// Call after `hal::init`, which enables EXTI vectors. Choose the wait with
/// `wait_for_high`, `wait_for_low`, `wait_for_rising_edge`,
/// `wait_for_falling_edge`, or `wait_for_any_edge`.
/// No interrupt is armed until a wait is polled. Dropping a pending wait masks
/// this input's line; other inputs on a shared vector remain available.
/// The priority applies to the entire `EXTI2` vector.
pub fn setup_primary_imu_drdy<'d, I>(
    int: hal::Peri<'d, pins::Imu1Int>,
    int_exti: hal::Peri<'d, hal::peripherals::EXTI2>,
    irqs: I,
    pull: hal::gpio::Pull,
    priority: hal::interrupt::Priority,
) -> ImuDrdyInput<'d>
where
    I: PrimaryImuDrdyIrqs,
{
    hal::interrupt::EXTI2.set_priority(priority);
    hal::exti::ExtiInput::new(int, int_exti, pull, irqs)
}

/// Embassy handler for the secondary IMU data-ready line (`PE15` / `EXTI15_10`).
///
/// Bind the vector once, including when other inputs share it. The same binding
/// also supports standard Embassy EXTI inputs on the other lines of that vector.
pub type SecondaryImuDrdyInterruptHandler =
    hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI15_10>;

pub trait SecondaryImuDrdyIrqs:
    hal::interrupt::typelevel::Binding<
        hal::interrupt::typelevel::EXTI15_10,
        SecondaryImuDrdyInterruptHandler,
    >
{
}

impl<T> SecondaryImuDrdyIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            hal::interrupt::typelevel::EXTI15_10,
            SecondaryImuDrdyInterruptHandler,
        >
{
}

/// Construct the secondary IMU async data-ready input (`PE15` / `EXTI15_10`).
///
/// Call after `hal::init`, which enables EXTI vectors. Choose the wait with
/// `wait_for_high`, `wait_for_low`, `wait_for_rising_edge`,
/// `wait_for_falling_edge`, or `wait_for_any_edge`.
/// No interrupt is armed until a wait is polled. Dropping a pending wait masks
/// this input's line; other inputs on a shared vector remain available.
/// The priority applies to the entire `EXTI15_10` vector.
pub fn setup_secondary_imu_drdy<'d, I>(
    int: hal::Peri<'d, pins::Imu2Int>,
    int_exti: hal::Peri<'d, hal::peripherals::EXTI15>,
    irqs: I,
    pull: hal::gpio::Pull,
    priority: hal::interrupt::Priority,
) -> ImuDrdyInput<'d>
where
    I: SecondaryImuDrdyIrqs,
{
    hal::interrupt::EXTI15_10.set_priority(priority);
    hal::exti::ExtiInput::new(int, int_exti, pull, irqs)
}

pub trait PrimaryImuIrqs:
    hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH0 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH0>,
    > + hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH1 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH1>,
    >
{
}

impl<T> PrimaryImuIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH0 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH0>,
        > + hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH1 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH1>,
        >
{
}

pub trait SecondaryImuIrqs:
    hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH6 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH6>,
    > + hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH7 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH7>,
    >
{
}

impl<T> SecondaryImuIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH6 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH6>,
        > + hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH7 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH7>,
        >
{
}

pub struct Leds<'d> {
    pub led0: hal::Peri<'d, pins::Led0>,
    pub led1: hal::Peri<'d, pins::Led1>,
}

pub struct PrimaryImu<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI1>,
    pub sck: hal::Peri<'d, pins::Imu1Sck>,
    pub miso: hal::Peri<'d, pins::Imu1Miso>,
    pub mosi: hal::Peri<'d, pins::Imu1Mosi>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI2>,
    pub dma_tx: hal::Peri<'d, hal::peripherals::DMA1_CH0>,
    pub dma_rx: hal::Peri<'d, hal::peripherals::DMA1_CH1>,
}

pub struct PrimaryImuSpi<'d> {
    pub spi: hal::spi::Spi<'d, hal::mode::Async, hal::spi::mode::Master>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI2>,
}

impl<'d> PrimaryImu<'d> {
    /// Construct async SPI using application-owned interrupt bindings.
    pub fn new_spi<I>(self, irqs: I, config: hal::spi::Config) -> PrimaryImuSpi<'d>
    where
        I: PrimaryImuIrqs + 'd,
    {
        PrimaryImuSpi {
            spi: hal::spi::Spi::new(
                self.spi,
                self.sck,
                self.mosi,
                self.miso,
                self.dma_tx,
                self.dma_rx,
                irqs,
                config,
            ),
            cs: self.cs,
            int: self.int,
            int_exti: self.int_exti,
        }
    }
}

pub struct SecondaryImu<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI4>,
    pub sck: hal::Peri<'d, pins::Imu2Sck>,
    pub miso: hal::Peri<'d, pins::Imu2Miso>,
    pub mosi: hal::Peri<'d, pins::Imu2Mosi>,
    pub cs: hal::Peri<'d, pins::Imu2Cs>,
    pub int: hal::Peri<'d, pins::Imu2Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI15>,
    pub dma_tx: hal::Peri<'d, hal::peripherals::DMA1_CH6>,
    pub dma_rx: hal::Peri<'d, hal::peripherals::DMA1_CH7>,
}

pub struct SecondaryImuSpi<'d> {
    pub spi: hal::spi::Spi<'d, hal::mode::Async, hal::spi::mode::Master>,
    pub cs: hal::Peri<'d, pins::Imu2Cs>,
    pub int: hal::Peri<'d, pins::Imu2Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI15>,
}

impl<'d> SecondaryImu<'d> {
    /// Construct async SPI using application-owned interrupt bindings.
    pub fn new_spi<I>(self, irqs: I, config: hal::spi::Config) -> SecondaryImuSpi<'d>
    where
        I: SecondaryImuIrqs + 'd,
    {
        SecondaryImuSpi {
            spi: hal::spi::Spi::new(
                self.spi,
                self.sck,
                self.mosi,
                self.miso,
                self.dma_tx,
                self.dma_rx,
                irqs,
                config,
            ),
            cs: self.cs,
            int: self.int,
            int_exti: self.int_exti,
        }
    }
}

pub struct OsdParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI2>,
    pub sck: hal::Peri<'d, pins::OsdSck>,
    pub miso: hal::Peri<'d, pins::OsdMiso>,
    pub mosi: hal::Peri<'d, pins::OsdMosi>,
    pub cs: hal::Peri<'d, pins::OsdCs>,
    pub dma_tx: hal::Peri<'d, hal::peripherals::DMA1_CH2>,
    pub dma_rx: hal::Peri<'d, hal::peripherals::DMA1_CH3>,
}

pub struct OsdSpi<'d> {
    pub spi: hal::spi::Spi<'d, hal::mode::Async, hal::spi::mode::Master>,
    pub cs: hal::Peri<'d, pins::OsdCs>,
}

pub trait OsdSpiIrqs:
    hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH2 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH2>,
    > + hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH3 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH3>,
    >
{
}

impl<T> OsdSpiIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH2 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH2>,
        > + hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH3 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH3>,
        >
{
}

impl<'d> OsdParts<'d> {
    /// Construct async SPI using application-owned interrupt bindings.
    pub fn new_spi<I>(self, irqs: I, config: hal::spi::Config) -> OsdSpi<'d>
    where
        I: OsdSpiIrqs + 'd,
    {
        OsdSpi {
            spi: hal::spi::Spi::new(
                self.spi,
                self.sck,
                self.mosi,
                self.miso,
                self.dma_tx,
                self.dma_rx,
                irqs,
                config,
            ),
            cs: self.cs,
        }
    }
}

pub struct Spi3Parts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI3>,
    pub sck: hal::Peri<'d, pins::Spi3Sck>,
    pub miso: hal::Peri<'d, pins::Spi3Miso>,
    pub mosi: hal::Peri<'d, pins::Spi3Mosi>,
}

/// Receiver UART, grouped identically to the general-purpose UARTs.
pub type ReceiverParts<'d> = UartPort<
    hal::Peri<'d, hal::peripherals::USART6>,
    hal::Peri<'d, pins::ReceiverTx>,
    hal::Peri<'d, pins::ReceiverRx>,
>;

pub struct UartPortsParts<'d> {
    pub uart1: UartPort<
        hal::Peri<'d, hal::peripherals::USART1>,
        hal::Peri<'d, pins::Uart1Tx>,
        hal::Peri<'d, pins::Uart1Rx>,
    >,
    pub uart2: UartPort<
        hal::Peri<'d, hal::peripherals::USART2>,
        hal::Peri<'d, pins::Uart2Tx>,
        hal::Peri<'d, pins::Uart2Rx>,
    >,
    pub uart3: UartPort<
        hal::Peri<'d, hal::peripherals::USART3>,
        hal::Peri<'d, pins::Uart3Tx>,
        hal::Peri<'d, pins::Uart3Rx>,
    >,
    pub uart4: UartPort<
        hal::Peri<'d, hal::peripherals::UART4>,
        hal::Peri<'d, pins::Uart4Tx>,
        hal::Peri<'d, pins::Uart4Rx>,
    >,
    pub uart7: UartPort<
        hal::Peri<'d, hal::peripherals::UART7>,
        hal::Peri<'d, pins::Uart7Tx>,
        hal::Peri<'d, pins::Uart7Rx>,
    >,
    pub uart8: UartPort<
        hal::Peri<'d, hal::peripherals::UART8>,
        hal::Peri<'d, pins::Uart8Tx>,
        hal::Peri<'d, pins::Uart8Rx>,
    >,
}

pub struct I2cParts<'d> {
    pub i2c1: hal::Peri<'d, hal::peripherals::I2C1>,
    pub i2c1_scl: hal::Peri<'d, pins::I2c1Scl>,
    pub i2c1_sda: hal::Peri<'d, pins::I2c1Sda>,
}

pub struct BaroParts<'d> {
    pub i2c2: hal::Peri<'d, hal::peripherals::I2C2>,
    pub i2c2_scl: hal::Peri<'d, pins::I2c2Scl>,
    pub i2c2_sda: hal::Peri<'d, pins::I2c2Sda>,
}

impl<'d> BaroParts<'d> {
    pub fn new_i2c_blocking(
        self,
        config: hal::i2c::Config,
    ) -> hal::i2c::I2c<'d, hal::mode::Blocking, hal::i2c::Master> {
        hal::i2c::I2c::new_blocking(self.i2c2, self.i2c2_scl, self.i2c2_sda, config)
    }
}

pub struct SdioParts<'d> {
    pub sdmmc1: hal::Peri<'d, hal::peripherals::SDMMC1>,
    pub ck: hal::Peri<'d, pins::SdioCk>,
    pub cmd: hal::Peri<'d, pins::SdioCmd>,
    pub d0: hal::Peri<'d, pins::SdioD0>,
    pub d1: hal::Peri<'d, pins::SdioD1>,
    pub d2: hal::Peri<'d, pins::SdioD2>,
    pub d3: hal::Peri<'d, pins::SdioD3>,
}

pub struct AuxParts<'d> {
    /// Servo 1/2: CH1/CH2, sharing one time base.
    pub tim15: hal::Peri<'d, hal::peripherals::TIM15>,
    /// LED strip: CH1.
    pub tim1: hal::Peri<'d, hal::peripherals::TIM1>,
    pub servo1: hal::Peri<'d, pins::Servo1>,
    pub servo2: hal::Peri<'d, pins::Servo2>,
    pub led_strip: hal::Peri<'d, pins::LedStrip>,
    pub beeper: hal::Peri<'d, pins::Beeper>,
    pub pinio1: hal::Peri<'d, pins::Pinio1>,
    pub pinio2: hal::Peri<'d, pins::Pinio2>,
    pub pinio3: hal::Peri<'d, pins::Pinio3>,
}

pub struct MotorParts<'d> {
    /// M7/M8: CH1/CH2.
    pub tim4: hal::Peri<'d, hal::peripherals::TIM4>,
    pub tim3: hal::Peri<'d, hal::peripherals::TIM3>,
    pub tim5: hal::Peri<'d, hal::peripherals::TIM5>,
    pub m1: hal::Peri<'d, pins::Motor1>,
    pub m2: hal::Peri<'d, pins::Motor2>,
    pub m3: hal::Peri<'d, pins::Motor3>,
    pub m4: hal::Peri<'d, pins::Motor4>,
    pub m5: hal::Peri<'d, pins::Motor5>,
    pub m6: hal::Peri<'d, pins::Motor6>,
    pub m7: hal::Peri<'d, pins::Motor7>,
    pub m8: hal::Peri<'d, pins::Motor8>,
}

pub struct AdcParts<'d> {
    pub adc1: hal::Peri<'d, hal::peripherals::ADC1>,
    pub adc3: hal::Peri<'d, hal::peripherals::ADC3>,
    pub vbat: hal::Peri<'d, pins::VbatAdc>,
    pub current: hal::Peri<'d, pins::CurrentAdc>,
    pub rssi: hal::Peri<'d, pins::RssiAdc>,
    pub external1: hal::Peri<'d, pins::External1Adc>,
    pub external2: hal::Peri<'d, pins::External2Adc>,
    pub external3: hal::Peri<'d, pins::External3Adc>,
}

pub struct UsbParts<'d> {
    pub otg_fs: hal::Peri<'d, hal::peripherals::USB_OTG_FS>,
    pub dm: hal::Peri<'d, pins::UsbDm>,
    pub dp: hal::Peri<'d, pins::UsbDp>,
}

// Metadata is transcribed from docs/tbs_lucid_config.h.
impl ImuMetadata for PrimaryImu<'_> {
    const ALIGNMENT: ImuAlignment = ImuAlignment::Cw90DegFlip;
}

impl PrimaryImu<'_> {
    pub const ALIGNMENT: ImuAlignment = <Self as ImuMetadata>::ALIGNMENT;
}

impl StorageCapability for SdioParts<'_> {
    const BUS: StorageBus = StorageBus::Sdmmc { data_lines: 4 };
}

impl SdioParts<'_> {
    pub const BUS: StorageBus = <Self as StorageCapability>::BUS;
}

impl AuxParts<'_> {
    pub const BEEPER_POLARITY: PinPolarity = PinPolarity::ActiveLow;
}

impl Leds<'_> {
    /// The target header does not specify LED polarity.
    pub const LED0_POLARITY: Option<PinPolarity> = None;
    pub const LED1_POLARITY: Option<PinPolarity> = None;
}

impl AdcParts<'_> {
    pub const SCALE: AnalogScale = AnalogScale {
        voltage_meter_scale: None,
        current_meter_scale: Some(250),
    };
}

impl<'d> BlockingI2c for BaroParts<'d> {
    type Config = hal::i2c::Config;
    type Bus = hal::i2c::I2c<'d, hal::mode::Blocking, hal::i2c::Master>;

    fn new_i2c_blocking(self, config: Self::Config) -> Self::Bus {
        BaroParts::new_i2c_blocking(self, config)
    }
}

impl ImuMetadata for SecondaryImu<'_> {
    const ALIGNMENT: ImuAlignment = ImuAlignment::Cw0DegFlip;
}

impl SecondaryImu<'_> {
    pub const ALIGNMENT: ImuAlignment = <Self as ImuMetadata>::ALIGNMENT;
}
