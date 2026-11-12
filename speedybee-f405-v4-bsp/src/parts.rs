use crate::core::capabilities::AnalogScale;
use crate::core::capabilities::BlockingI2c;
use crate::core::capabilities::ImuAlignment;
use crate::core::capabilities::ImuMetadata;
use crate::core::capabilities::PinPolarity;
use crate::core::capabilities::RxOnlyUartPort;
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

/// Embassy handler for the primary IMU data-ready line (`PC4` / `EXTI4`).
///
/// Bind the vector once, including when other inputs share it. The same binding
/// also supports standard Embassy EXTI inputs on the other lines of that vector.
pub type PrimaryImuDrdyInterruptHandler =
    hal::exti::InterruptHandler<hal::interrupt::typelevel::EXTI4>;

pub trait PrimaryImuDrdyIrqs:
    hal::interrupt::typelevel::Binding<hal::interrupt::typelevel::EXTI4, PrimaryImuDrdyInterruptHandler>
{
}

impl<T> PrimaryImuDrdyIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            hal::interrupt::typelevel::EXTI4,
            PrimaryImuDrdyInterruptHandler,
        >
{
}

/// Construct the primary IMU async data-ready input (`PC4` / `EXTI4`).
///
/// Call after `hal::init`, which enables EXTI vectors. Choose the wait with
/// `wait_for_high`, `wait_for_low`, `wait_for_rising_edge`,
/// `wait_for_falling_edge`, or `wait_for_any_edge`.
/// No interrupt is armed until a wait is polled. Dropping a pending wait masks
/// this input's line; other inputs on a shared vector remain available.
/// The priority applies to the entire `EXTI4` vector.
pub fn setup_primary_imu_drdy<'d, I>(
    int: hal::Peri<'d, pins::Imu1Int>,
    int_exti: hal::Peri<'d, hal::peripherals::EXTI4>,
    irqs: I,
    pull: hal::gpio::Pull,
    priority: hal::interrupt::Priority,
) -> ImuDrdyInput<'d>
where
    I: PrimaryImuDrdyIrqs,
{
    hal::interrupt::EXTI4.set_priority(priority);
    hal::exti::ExtiInput::new(int, int_exti, pull, irqs)
}

pub trait PrimaryImuIrqs:
    hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA2_CH3 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA2_CH3>,
    > + hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA2_CH2 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA2_CH2>,
    >
{
}

impl<T> PrimaryImuIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA2_CH3 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA2_CH3>,
        > + hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA2_CH2 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA2_CH2>,
        >
{
}

pub struct Leds<'d> {
    pub led0: hal::Peri<'d, pins::Led0>,
}

pub struct PrimaryImu<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI1>,
    pub sck: hal::Peri<'d, pins::Imu1Sck>,
    pub miso: hal::Peri<'d, pins::Imu1Miso>,
    pub mosi: hal::Peri<'d, pins::Imu1Mosi>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI4>,
    pub dma_tx: hal::Peri<'d, hal::peripherals::DMA2_CH3>,
    pub dma_rx: hal::Peri<'d, hal::peripherals::DMA2_CH2>,
}

pub struct PrimaryImuSpi<'d> {
    pub spi: hal::spi::Spi<'d, hal::mode::Async, hal::spi::mode::Master>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI4>,
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

pub struct OsdParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI2>,
    pub sck: hal::Peri<'d, pins::OsdSck>,
    pub miso: hal::Peri<'d, pins::OsdMiso>,
    pub mosi: hal::Peri<'d, pins::OsdMosi>,
    pub cs: hal::Peri<'d, pins::OsdCs>,
    pub dma_tx: hal::Peri<'d, hal::peripherals::DMA1_CH4>,
    pub dma_rx: hal::Peri<'d, hal::peripherals::DMA1_CH3>,
}

pub struct OsdSpi<'d> {
    pub spi: hal::spi::Spi<'d, hal::mode::Async, hal::spi::mode::Master>,
    pub cs: hal::Peri<'d, pins::OsdCs>,
}

pub trait OsdSpiIrqs:
    hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH4 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH4>,
    > + hal::interrupt::typelevel::Binding<
        <hal::peripherals::DMA1_CH3 as hal::dma::ChannelInstance>::Interrupt,
        hal::dma::InterruptHandler<hal::peripherals::DMA1_CH3>,
    >
{
}

impl<T> OsdSpiIrqs for T where
    T: hal::interrupt::typelevel::Binding<
            <hal::peripherals::DMA1_CH4 as hal::dma::ChannelInstance>::Interrupt,
            hal::dma::InterruptHandler<hal::peripherals::DMA1_CH4>,
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

pub struct SdcardSpiParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI3>,
    pub sck: hal::Peri<'d, pins::SdcardSck>,
    pub miso: hal::Peri<'d, pins::SdcardMiso>,
    pub mosi: hal::Peri<'d, pins::SdcardMosi>,
    pub cs: hal::Peri<'d, pins::SdcardCs>,
}

/// Receiver UART, grouped identically to the general-purpose UARTs.
pub type ReceiverParts<'d> = UartPort<
    hal::Peri<'d, hal::peripherals::USART2>,
    hal::Peri<'d, pins::ReceiverTx>,
    hal::Peri<'d, pins::ReceiverRx>,
>;

pub struct UartPortsParts<'d> {
    pub uart1: UartPort<
        hal::Peri<'d, hal::peripherals::USART1>,
        hal::Peri<'d, pins::Uart1Tx>,
        hal::Peri<'d, pins::Uart1Rx>,
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
    pub uart5: RxOnlyUartPort<hal::Peri<'d, hal::peripherals::UART5>, hal::Peri<'d, pins::Uart5Rx>>,
    pub uart6: UartPort<
        hal::Peri<'d, hal::peripherals::USART6>,
        hal::Peri<'d, pins::Uart6Tx>,
        hal::Peri<'d, pins::Uart6Rx>,
    >,
}

pub struct UsbParts<'d> {
    pub otg_fs: hal::Peri<'d, hal::peripherals::USB_OTG_FS>,
    pub dm: hal::Peri<'d, pins::UsbDm>,
    pub dp: hal::Peri<'d, pins::UsbDp>,
}

pub struct BaroParts<'d> {
    pub i2c1: hal::Peri<'d, hal::peripherals::I2C1>,
    pub i2c1_scl: hal::Peri<'d, pins::I2c1Scl>,
    pub i2c1_sda: hal::Peri<'d, pins::I2c1Sda>,
}

impl<'d> BaroParts<'d> {
    pub fn new_i2c_blocking(
        self,
        config: hal::i2c::Config,
    ) -> hal::i2c::I2c<'d, hal::mode::Blocking, hal::i2c::Master> {
        hal::i2c::I2c::new_blocking(self.i2c1, self.i2c1_scl, self.i2c1_sda, config)
    }
}

pub struct MotorParts<'d> {
    /// M1/M2: CH1/CH2.
    pub tim4: hal::Peri<'d, hal::peripherals::TIM4>,
    /// M3/M4: CH3/CH4.
    pub tim3: hal::Peri<'d, hal::peripherals::TIM3>,
    /// M5/M6: CH3/CH4; also supports aux complementary outputs.
    pub tim8: hal::Peri<'d, hal::peripherals::TIM8>,
    /// M7/M8: CH3/CH1.
    pub tim2: hal::Peri<'d, hal::peripherals::TIM2>,
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
    pub vbat: hal::Peri<'d, pins::VbatAdc>,
    pub current: hal::Peri<'d, pins::CurrentAdc>,
    pub rssi: hal::Peri<'d, pins::RssiAdc>,
}

pub struct AuxParts<'d> {
    /// LED strip: CH1.
    pub tim1: hal::Peri<'d, hal::peripherals::TIM1>,
    /// Camera/servo: CH1/CH2 at AF9, sharing one time base.
    pub tim12: hal::Peri<'d, hal::peripherals::TIM12>,
    pub beeper: hal::Peri<'d, pins::Beeper>,
    pub servo1: hal::Peri<'d, pins::Servo1>,
    pub led_strip: hal::Peri<'d, pins::LedStrip>,
    pub camera_control: hal::Peri<'d, pins::CameraControl>,
    pub pinio1: hal::Peri<'d, pins::Pinio1>,
}

// Metadata is transcribed from docs/speedybee_config.h.
impl ImuMetadata for PrimaryImu<'_> {
    const ALIGNMENT: ImuAlignment = ImuAlignment::Cw90Deg;
}

impl PrimaryImu<'_> {
    pub const ALIGNMENT: ImuAlignment = <Self as ImuMetadata>::ALIGNMENT;
}

impl StorageCapability for SdcardSpiParts<'_> {
    const BUS: StorageBus = StorageBus::Spi;
}

impl SdcardSpiParts<'_> {
    pub const BUS: StorageBus = <Self as StorageCapability>::BUS;
}

impl AuxParts<'_> {
    pub const BEEPER_POLARITY: PinPolarity = PinPolarity::ActiveLow;
}

impl Leds<'_> {
    /// The target header does not specify LED polarity.
    pub const LED0_POLARITY: Option<PinPolarity> = None;
}

impl AdcParts<'_> {
    pub const SCALE: AnalogScale = AnalogScale {
        voltage_meter_scale: None,
        current_meter_scale: Some(400),
    };
}

impl<'d> BlockingI2c for BaroParts<'d> {
    type Config = hal::i2c::Config;
    type Bus = hal::i2c::I2c<'d, hal::mode::Blocking, hal::i2c::Master>;

    fn new_i2c_blocking(self, config: Self::Config) -> Self::Bus {
        BaroParts::new_i2c_blocking(self, config)
    }
}
