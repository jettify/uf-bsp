use crate::hal;
use crate::pins;

pub struct Leds<'d> {
    pub led0: hal::Peri<'d, pins::Led0>,
    pub led1: hal::Peri<'d, pins::Led1>,
}

pub struct ImuPrimaryParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI1>,
    pub sck: hal::Peri<'d, pins::Imu1Sck>,
    pub miso: hal::Peri<'d, pins::Imu1Miso>,
    pub mosi: hal::Peri<'d, pins::Imu1Mosi>,
    pub tx_dma: hal::Peri<'d, hal::peripherals::DMA1_CH3>,
    pub rx_dma: hal::Peri<'d, hal::peripherals::DMA1_CH4>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI2>,
}

pub struct ImuSecondaryParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI4>,
    pub sck: hal::Peri<'d, pins::Imu2Sck>,
    pub miso: hal::Peri<'d, pins::Imu2Miso>,
    pub mosi: hal::Peri<'d, pins::Imu2Mosi>,
    pub tx_dma: hal::Peri<'d, hal::peripherals::DMA1_CH6>,
    pub rx_dma: hal::Peri<'d, hal::peripherals::DMA1_CH7>,
    pub cs: hal::Peri<'d, pins::Imu2Cs>,
    pub int: hal::Peri<'d, pins::Imu2Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI15>,
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
    pub vbat: hal::Peri<'d, pins::VbatAdc>,
    pub current: hal::Peri<'d, pins::CurrentAdc>,
    pub rssi: hal::Peri<'d, pins::RssiAdc>,
}

pub struct UsbParts<'d> {
    pub otg_fs: hal::Peri<'d, hal::peripherals::USB_OTG_FS>,
    pub dm: hal::Peri<'d, pins::UsbDm>,
    pub dp: hal::Peri<'d, pins::UsbDp>,
}
