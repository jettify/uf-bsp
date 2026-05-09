pub struct Leds<'d> {
    pub status_led: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> Leds<'d> {
    pub fn new() -> Self {
        Self {
            status_led: None,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct ImuParts<'d> {
    pub cs: Option<()>,
    pub int: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> ImuParts<'d> {
    pub fn new() -> Self {
        Self {
            cs: None,
            int: None,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct ReceiverParts<'d> {
    pub tx: Option<()>,
    pub rx: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> ReceiverParts<'d> {
    pub fn new() -> Self {
        Self {
            tx: None,
            rx: None,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct MotorParts<'d> {
    pub ch1: Option<()>,
    pub ch2: Option<()>,
    pub ch3: Option<()>,
    pub ch4: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> MotorParts<'d> {
    pub fn new() -> Self {
        Self {
            ch1: None,
            ch2: None,
            ch3: None,
            ch4: None,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct UsbParts<'d> {
    pub dm: Option<()>,
    pub dp: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> UsbParts<'d> {
    pub fn new() -> Self {
        Self {
            dm: None,
            dp: None,
            _marker: core::marker::PhantomData,
        }
    }
}

pub struct AdcParts<'d> {
    pub battery_adc: Option<()>,
    _marker: core::marker::PhantomData<&'d ()>,
}

impl<'d> AdcParts<'d> {
    pub fn new() -> Self {
        Self {
            battery_adc: None,
            _marker: core::marker::PhantomData,
        }
    }
}
