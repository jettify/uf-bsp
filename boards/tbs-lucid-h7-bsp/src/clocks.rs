use crate::hal;
use crate::hal::rcc::AHBPrescaler;
use crate::hal::rcc::APBPrescaler;
use crate::hal::rcc::Hse;
use crate::hal::rcc::HseMode;
use crate::hal::rcc::Pll;
use crate::hal::rcc::PllDiv;
use crate::hal::rcc::PllMul;
use crate::hal::rcc::PllPreDiv;
use crate::hal::rcc::PllSource;
use crate::hal::rcc::Sysclk;
use crate::hal::rcc::TimerPrescaler;
use crate::hal::rcc::VoltageScale;
use crate::hal::time::Hertz;

pub fn config() -> hal::Config {
    let mut cfg = hal::Config::default();

    cfg.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });

    cfg.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL120,
        fracn: None,
        divp: Some(PllDiv::DIV2),
        divq: Some(PllDiv::DIV5),
        divr: Some(PllDiv::DIV5),
    });
    cfg.rcc.sys = Sysclk::PLL1_P;

    cfg.rcc.d1c_pre = AHBPrescaler::DIV1;
    cfg.rcc.ahb_pre = AHBPrescaler::DIV2;
    cfg.rcc.apb1_pre = APBPrescaler::DIV2;
    cfg.rcc.apb2_pre = APBPrescaler::DIV2;
    cfg.rcc.apb3_pre = APBPrescaler::DIV2;
    cfg.rcc.apb4_pre = APBPrescaler::DIV2;
    cfg.rcc.timer_prescaler = TimerPrescaler::DefaultX2;
    cfg.rcc.voltage_scale = VoltageScale::Scale0;

    cfg
}

pub fn config_with_usb() -> hal::Config {
    let mut cfg = config();
    cfg.rcc.hsi48 = Some(hal::rcc::Hsi48Config {
        sync_from_usb: true,
    });
    cfg
}
