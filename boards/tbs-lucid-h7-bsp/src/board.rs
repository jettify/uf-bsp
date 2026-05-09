use crate::hal;
use crate::parts::{AdcParts, ImuParts, Leds, MotorParts, ReceiverParts, UsbParts};

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub imu: ImuParts<'d>,
    pub receiver: ReceiverParts<'d>,
    pub motors: MotorParts<'d>,
    pub usb: UsbParts<'d>,
    pub adc: AdcParts<'d>,
}

impl<'d> Board<'d> {
    pub fn new(_p: hal::Peripherals) -> Self {
        Self {
            leds: Leds::new(),
            imu: ImuParts::new(),
            receiver: ReceiverParts::new(),
            motors: MotorParts::new(),
            usb: UsbParts::new(),
            adc: AdcParts::new(),
        }
    }
}
