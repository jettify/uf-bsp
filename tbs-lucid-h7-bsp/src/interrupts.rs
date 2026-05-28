//! Embassy-compatible interrupt bindings for TBS Lucid H7 peripherals.

use crate::hal;

hal::bind_interrupts!(pub struct ReceiverUartIrqs {
    USART6 => hal::usart::InterruptHandler<hal::peripherals::USART6>;
});

hal::bind_interrupts!(pub struct UsbFsIrqs {
    OTG_FS => hal::usb::InterruptHandler<hal::peripherals::USB_OTG_FS>;
});

hal::bind_interrupts!(pub struct PrimaryImuSpiIrqs {
    DMA1_STREAM0 => hal::dma::InterruptHandler<hal::peripherals::DMA1_CH0>;
    DMA1_STREAM1 => hal::dma::InterruptHandler<hal::peripherals::DMA1_CH1>;
});

hal::bind_interrupts!(pub struct SecondaryImuSpiIrqs {
    DMA1_STREAM6 => hal::dma::InterruptHandler<hal::peripherals::DMA1_CH6>;
    DMA1_STREAM7 => hal::dma::InterruptHandler<hal::peripherals::DMA1_CH7>;
});
