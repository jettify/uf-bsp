#![no_std]
#![no_main]

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_halt as _;
use tbs_lucid_h7_bsp as bsp;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::High, Speed::Low);

    loop {
        led0.toggle();
        led1.toggle();
        Timer::after(Duration::from_millis(250)).await;
    }
}
