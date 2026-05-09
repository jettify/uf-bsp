use crate::hal;

pub struct Leds<'d> {
    pub led0: hal::Peri<'d, hal::peripherals::PE3>,
    pub led1: hal::Peri<'d, hal::peripherals::PE4>,
}

pub struct ImuPrimaryParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI1>,
    pub sck: hal::Peri<'d, hal::peripherals::PA5>,
    pub miso: hal::Peri<'d, hal::peripherals::PA6>,
    pub mosi: hal::Peri<'d, hal::peripherals::PD7>,
    pub cs: hal::Peri<'d, hal::peripherals::PC15>,
    pub int: hal::Peri<'d, hal::peripherals::PB2>,
}

pub struct ImuSecondaryParts<'d> {
    pub spi: hal::Peri<'d, hal::peripherals::SPI4>,
    pub sck: hal::Peri<'d, hal::peripherals::PE12>,
    pub miso: hal::Peri<'d, hal::peripherals::PE13>,
    pub mosi: hal::Peri<'d, hal::peripherals::PE14>,
    pub cs: hal::Peri<'d, hal::peripherals::PE11>,
    pub int: hal::Peri<'d, hal::peripherals::PE15>,
}

pub struct ReceiverParts<'d> {
    pub uart: hal::Peri<'d, hal::peripherals::USART6>,
    pub tx: hal::Peri<'d, hal::peripherals::PC6>,
    pub rx: hal::Peri<'d, hal::peripherals::PC7>,
}

pub struct MotorParts<'d> {
    pub tim3: hal::Peri<'d, hal::peripherals::TIM3>,
    pub tim5: hal::Peri<'d, hal::peripherals::TIM5>,
    pub m1: hal::Peri<'d, hal::peripherals::PB0>,
    pub m2: hal::Peri<'d, hal::peripherals::PB1>,
    pub m3: hal::Peri<'d, hal::peripherals::PA0>,
    pub m4: hal::Peri<'d, hal::peripherals::PA1>,
}

pub struct AdcParts<'d> {
    pub adc1: hal::Peri<'d, hal::peripherals::ADC1>,
    pub vbat: hal::Peri<'d, hal::peripherals::PC0>,
    pub current: hal::Peri<'d, hal::peripherals::PC1>,
    pub rssi: hal::Peri<'d, hal::peripherals::PC5>,
}
