//! Embassy-compatible interrupt bindings for `SpeedyBee` F405 V4 peripherals.

use crate::hal;

hal::bind_interrupts!(pub struct ReceiverUartIrqs {
    USART2 => hal::usart::InterruptHandler<hal::peripherals::USART2>;
});

hal::bind_interrupts!(pub struct UsbFsIrqs {
    OTG_FS => hal::usb::InterruptHandler<hal::peripherals::USB_OTG_FS>;
});

hal::bind_interrupts!(pub struct PrimaryImuSpiIrqs {
    DMA2_STREAM3 => hal::dma::InterruptHandler<hal::peripherals::DMA2_CH3>;
    DMA2_STREAM0 => hal::dma::InterruptHandler<hal::peripherals::DMA2_CH0>;
});
