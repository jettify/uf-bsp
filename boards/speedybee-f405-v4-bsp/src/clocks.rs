use crate::hal;
use crate::hal::rcc::AHBPrescaler;
use crate::hal::rcc::APBPrescaler;
use crate::hal::rcc::Hse;
use crate::hal::rcc::HseMode;
use crate::hal::rcc::Pll;
use crate::hal::rcc::PllPDiv;
use crate::hal::rcc::PllPreDiv;
use crate::hal::rcc::PllQDiv;
use crate::hal::rcc::PllRDiv;
use crate::hal::rcc::PllSource;
use crate::hal::rcc::Sysclk;
use crate::hal::time::Hertz;

pub fn config() -> hal::Config {
    let mut cfg = hal::Config::default();
    cfg.rcc.hsi = false;

    cfg.rcc.hse = Some(Hse {
        freq: Hertz(8_000_000),
        mode: HseMode::Oscillator,
    });
    cfg.rcc.pll_src = PllSource::HSE;

    cfg.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV8,
        mul: crate::hal::rcc::PllMul::MUL336,
        divp: Some(PllPDiv::DIV2),
        divq: Some(PllQDiv::DIV7),
        divr: Some(PllRDiv::DIV2),
    });
    cfg.rcc.sys = Sysclk::PLL1_P;

    cfg.rcc.ahb_pre = AHBPrescaler::DIV1;
    cfg.rcc.apb1_pre = APBPrescaler::DIV4;
    cfg.rcc.apb2_pre = APBPrescaler::DIV2;

    cfg
}
