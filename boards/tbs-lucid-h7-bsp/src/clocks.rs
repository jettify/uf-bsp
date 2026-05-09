use crate::hal;
use crate::hal::rcc::{
    AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllDiv, PllMul, PllPreDiv, PllSource, Sysclk,
    TimerPrescaler, VoltageScale,
};
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
