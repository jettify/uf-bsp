use crate::hal;

pub struct Spi1Dma<'d> {
    pub tx: hal::Peri<'d, hal::peripherals::DMA2_CH3>,
    pub rx: hal::Peri<'d, hal::peripherals::DMA2_CH0>,
}

pub struct Spi2Dma<'d> {
    pub tx: hal::Peri<'d, hal::peripherals::DMA1_CH4>,
    pub rx: hal::Peri<'d, hal::peripherals::DMA1_CH3>,
}

pub struct Spi3Dma<'d> {
    pub tx: hal::Peri<'d, hal::peripherals::DMA1_CH5>,
    pub rx: hal::Peri<'d, hal::peripherals::DMA1_CH0>,
}

pub struct Adc1Dma<'d> {
    pub ch: Option<hal::Peri<'d, hal::peripherals::DMA2_CH0>>,
}

pub struct Usart2Dma<'d> {
    pub tx: Option<hal::Peri<'d, hal::peripherals::DMA1_CH6>>,
    pub rx: Option<hal::Peri<'d, hal::peripherals::DMA1_CH5>>,
}

pub struct DmaResources<'d> {
    pub spi1: Spi1Dma<'d>,
    pub spi2: Spi2Dma<'d>,
    pub spi3: Spi3Dma<'d>,
    pub adc1: Adc1Dma<'d>,
    pub usart2: Usart2Dma<'d>,
}
