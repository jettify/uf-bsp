use crate::hal;

pub struct Adc1Dma<'d> {
    pub ch: hal::Peri<'d, hal::peripherals::DMA2_CH1>,
}

pub struct Adc3Dma<'d> {
    pub ch: hal::Peri<'d, hal::peripherals::DMA2_CH2>,
}

pub struct Usart6Dma<'d> {
    pub tx: Option<hal::Peri<'d, hal::peripherals::DMA2_CH5>>,
    pub rx: Option<hal::Peri<'d, hal::peripherals::DMA2_CH6>>,
}

pub struct DmaResources<'d> {
    pub adc1: Adc1Dma<'d>,
    pub adc3: Adc3Dma<'d>,
    pub usart6: Usart6Dma<'d>,
}
