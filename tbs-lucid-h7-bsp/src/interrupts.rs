//! Embassy-compatible interrupt bindings for TBS Lucid H7 peripherals.

use crate::hal;

hal::bind_interrupts!(pub struct ReceiverUartIrqs {
    USART6 => hal::usart::InterruptHandler<hal::peripherals::USART6>;
});

hal::bind_interrupts!(pub struct UsbFsIrqs {
    OTG_FS => hal::usb::InterruptHandler<hal::peripherals::USB_OTG_FS>;
});
