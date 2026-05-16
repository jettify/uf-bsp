#![no_std]
#![no_main]

use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

fn hz(v: bsp::hal::time::MaybeHertz) -> u32 {
    v.to_hertz().map(|x| x.0).unwrap_or(0)
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config());

    let c = bsp::hal::rcc::clocks(&p.RCC);

    let sys = hz(c.sys);
    let hclk1 = hz(c.hclk1);
    let pclk1 = hz(c.pclk1);
    let pclk2 = hz(c.pclk2);
    let pclk3 = hz(c.pclk3);
    let pclk4 = hz(c.pclk4);

    info!("clock_report");
    info!(
        "src: hse={} hsi={} csi={} hsi48={}",
        hz(c.hse),
        hz(c.hsi),
        hz(c.csi),
        hz(c.hsi48)
    );
    info!(
        "pll: pll1_q={} pll2_p={} pll2_q={} pll2_r={} pll3_p={} pll3_q={} pll3_r={}",
        hz(c.pll1_q),
        hz(c.pll2_p),
        hz(c.pll2_q),
        hz(c.pll2_r),
        hz(c.pll3_p),
        hz(c.pll3_q),
        hz(c.pll3_r)
    );
    info!(
        "bus: sys={} hclk1={} hclk2={} hclk3={} hclk4={}",
        sys,
        hclk1,
        hz(c.hclk2),
        hz(c.hclk3),
        hz(c.hclk4)
    );
    info!(
        "apb: pclk1={} pclk2={} pclk3={} pclk4={}",
        pclk1, pclk2, pclk3, pclk4
    );

    let sys_hclk1 = sys.checked_div(hclk1).unwrap_or(0);
    let hclk1_pclk1 = hclk1.checked_div(pclk1).unwrap_or(0);
    let hclk1_pclk2 = hclk1.checked_div(pclk2).unwrap_or(0);
    let hclk1_pclk3 = hclk1.checked_div(pclk3).unwrap_or(0);
    let hclk1_pclk4 = hclk1.checked_div(pclk4).unwrap_or(0);

    info!(
        "ratio: sys:hclk1={} hclk1:pclk1={} hclk1:pclk2={} hclk1:pclk3={} hclk1:pclk4={}",
        sys_hclk1, hclk1_pclk1, hclk1_pclk2, hclk1_pclk3, hclk1_pclk4
    );

    loop {
        Timer::after(Duration::from_secs(10)).await;
    }
}
