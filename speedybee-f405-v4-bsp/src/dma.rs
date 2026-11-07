use crate::hal;

pub struct Spi3Dma<'d> {
    pub tx: hal::Peri<'d, hal::peripherals::DMA1_CH5>,
    pub rx: hal::Peri<'d, hal::peripherals::DMA1_CH0>,
}

pub struct Adc1Dma<'d> {
    pub ch: Option<hal::Peri<'d, hal::peripherals::DMA2_CH0>>,
}

pub struct Usart2Dma<'d> {
    /// USART2 TX stream.
    pub tx: hal::Peri<'d, hal::peripherals::DMA1_CH6>,
    /// Unavailable by default: SPI3 TX owns DMA1_CH5. Move
    /// `board.dma.spi3.tx` into a USART2 RX driver to give up SPI3 TX DMA.
    pub rx: Option<hal::Peri<'d, hal::peripherals::DMA1_CH5>>,
}

pub struct DmaResources<'d> {
    pub spi3: Spi3Dma<'d>,
    pub adc1: Adc1Dma<'d>,
    pub usart2: Usart2Dma<'d>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DmaLayout {
    ImuSpiPreferred,
    Adc1Preferred,
}
