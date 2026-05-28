use crate::hal;
use crate::pins;

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
    pub fn new_spi(self, config: hal::spi::Config) -> PrimaryImuSpi<'d> {
        self.new_spi_with_irqs(crate::interrupts::PrimaryImuSpiIrqs, config)
    }

    pub fn new_spi_with_irqs<I>(self, irqs: I, config: hal::spi::Config) -> PrimaryImuSpi<'d>
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
    pub fn new_spi(self, config: hal::spi::Config) -> SecondaryImuSpi<'d> {
        self.new_spi_with_irqs(crate::interrupts::SecondaryImuSpiIrqs, config)
    }

    pub fn new_spi_with_irqs<I>(self, irqs: I, config: hal::spi::Config) -> SecondaryImuSpi<'d>
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
}

pub struct Spi3Parts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI3>,
    pub sck: hal::Peri<'d, pins::Spi3Sck>,
    pub miso: hal::Peri<'d, pins::Spi3Miso>,
    pub mosi: hal::Peri<'d, pins::Spi3Mosi>,
}

pub struct ReceiverParts<'d> {
    pub uart: hal::Peri<'d, hal::peripherals::USART6>,
    pub tx: hal::Peri<'d, pins::ReceiverTx>,
    pub rx: hal::Peri<'d, pins::ReceiverRx>,
}

pub struct UartPortsParts<'d> {
    pub uart1: hal::Peri<'d, hal::peripherals::USART1>,
    pub uart1_tx: hal::Peri<'d, pins::Uart1Tx>,
    pub uart1_rx: hal::Peri<'d, pins::Uart1Rx>,
    pub uart2: hal::Peri<'d, hal::peripherals::USART2>,
    pub uart2_tx: hal::Peri<'d, pins::Uart2Tx>,
    pub uart2_rx: hal::Peri<'d, pins::Uart2Rx>,
    pub uart3: hal::Peri<'d, hal::peripherals::USART3>,
    pub uart3_tx: hal::Peri<'d, pins::Uart3Tx>,
    pub uart3_rx: hal::Peri<'d, pins::Uart3Rx>,
    pub uart4: hal::Peri<'d, hal::peripherals::UART4>,
    pub uart4_tx: hal::Peri<'d, pins::Uart4Tx>,
    pub uart4_rx: hal::Peri<'d, pins::Uart4Rx>,
    pub uart7: hal::Peri<'d, hal::peripherals::UART7>,
    pub uart7_tx: hal::Peri<'d, pins::Uart7Tx>,
    pub uart7_rx: hal::Peri<'d, pins::Uart7Rx>,
    pub uart8: hal::Peri<'d, hal::peripherals::UART8>,
    pub uart8_tx: hal::Peri<'d, pins::Uart8Tx>,
    pub uart8_rx: hal::Peri<'d, pins::Uart8Rx>,
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
    pub servo1: hal::Peri<'d, pins::Servo1>,
    pub servo2: hal::Peri<'d, pins::Servo2>,
    pub led_strip: hal::Peri<'d, pins::LedStrip>,
    pub beeper: hal::Peri<'d, pins::Beeper>,
    pub pinio1: hal::Peri<'d, pins::Pinio1>,
    pub pinio2: hal::Peri<'d, pins::Pinio2>,
    pub pinio3: hal::Peri<'d, pins::Pinio3>,
}

pub struct MotorParts<'d> {
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
