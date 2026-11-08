use crate::core::irq_latch::IrqLatch;
use crate::core::irq_latch::IrqLatchClaimError;
use crate::core::irq_latch::IrqLatchState;
use crate::hal;
use crate::hal::interrupt::InterruptExt as _;
use crate::hal::interrupt::typelevel::Interrupt as _;
use crate::pac;
use crate::pins;

static PRIMARY_IMU_DRDY_STATE: IrqLatchState = IrqLatchState::new();

pub type ImuDrdyLatch<'d> = IrqLatch<hal::exti::ExtiInput<'d, hal::mode::Blocking>>;

/// Interrupt handler for the primary IMU data-ready line (`PA4` / `EXTI4`).
pub struct PrimaryImuDrdyInterruptHandler;

impl hal::interrupt::typelevel::Handler<hal::interrupt::typelevel::EXTI4>
    for PrimaryImuDrdyInterruptHandler
{
    unsafe fn on_interrupt() {
        pac::EXTI.pr(0).write(|w| w.set_line(4, true));
        PRIMARY_IMU_DRDY_STATE.on_irq();
    }
}

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

/// Configure and enable the primary IMU data-ready (`EXTI4`) interrupt latch.
///
/// Returns an error if the latch state already has a consumer. On error, the configured
/// input is dropped and this helper does not enable the NVIC interrupt.
pub fn setup_primary_imu_drdy<'d, I>(
    int: hal::Peri<'d, pins::Imu1Int>,
    int_exti: hal::Peri<'d, hal::peripherals::EXTI4>,
    _irqs: I,
    pull: hal::gpio::Pull,
    edge: hal::exti::TriggerEdge,
    priority: hal::interrupt::Priority,
) -> Result<ImuDrdyLatch<'d>, IrqLatchClaimError>
where
    I: PrimaryImuDrdyIrqs,
{
    hal::interrupt::typelevel::EXTI4::unpend();
    let input = hal::exti::ExtiInput::new_blocking(int, int_exti, pull, edge);
    let latch = IrqLatch::try_new(input, &PRIMARY_IMU_DRDY_STATE).map_err(|(_input, err)| err)?;
    hal::interrupt::EXTI4.set_priority(priority);
    // SAFETY: `I: PrimaryImuDrdyIrqs` proves `EXTI4` is bound to `PrimaryImuDrdyInterruptHandler`,
    // which clears the pending bit on line 4, and `input` has configured `EXTI4`.
    unsafe { hal::interrupt::typelevel::EXTI4::enable() };
    Ok(latch)
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

pub struct ReceiverParts<'d> {
    pub uart: hal::Peri<'d, hal::peripherals::USART2>,
    pub tx: hal::Peri<'d, pins::ReceiverTx>,
    pub rx: hal::Peri<'d, pins::ReceiverRx>,
}

pub struct UartPortsParts<'d> {
    pub uart1: hal::Peri<'d, hal::peripherals::USART1>,
    pub uart1_tx: hal::Peri<'d, pins::Uart1Tx>,
    pub uart1_rx: hal::Peri<'d, pins::Uart1Rx>,
    pub uart3: hal::Peri<'d, hal::peripherals::USART3>,
    pub uart3_tx: hal::Peri<'d, pins::Uart3Tx>,
    pub uart3_rx: hal::Peri<'d, pins::Uart3Rx>,
    pub uart4: hal::Peri<'d, hal::peripherals::UART4>,
    pub uart4_tx: hal::Peri<'d, pins::Uart4Tx>,
    pub uart4_rx: hal::Peri<'d, pins::Uart4Rx>,
    pub uart5: hal::Peri<'d, hal::peripherals::UART5>,
    pub uart5_rx: hal::Peri<'d, pins::Uart5Rx>,
    pub uart6: hal::Peri<'d, hal::peripherals::USART6>,
    pub uart6_tx: hal::Peri<'d, pins::Uart6Tx>,
    pub uart6_rx: hal::Peri<'d, pins::Uart6Rx>,
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
