use crate::hal;
use crate::pins;

pub struct Leds<'d> {
    pub led0: hal::Peri<'d, pins::Led0>,
}

pub struct ImuPrimaryParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI1>,
    pub sck: hal::Peri<'d, pins::Imu1Sck>,
    pub miso: hal::Peri<'d, pins::Imu1Miso>,
    pub mosi: hal::Peri<'d, pins::Imu1Mosi>,
    pub cs: hal::Peri<'d, pins::Imu1Cs>,
    pub int: hal::Peri<'d, pins::Imu1Int>,
    pub int_exti: hal::Peri<'d, hal::peripherals::EXTI4>,
}

pub struct OsdParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI2>,
    pub sck: hal::Peri<'d, pins::OsdSck>,
    pub miso: hal::Peri<'d, pins::OsdMiso>,
    pub mosi: hal::Peri<'d, pins::OsdMosi>,
    pub cs: hal::Peri<'d, pins::OsdCs>,
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

pub struct MotorParts<'d> {
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
