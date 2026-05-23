#![no_std]
#![no_main]

#[path = "support/usb_logger.rs"]
mod usb_logger;

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config_with_usb());
    let board = bsp::Board::new(p);
    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let log = usb_logger::spawn_default(&spawner, board.usb, "SpeedyBee F405 USB Logger");
    log.wait_startup().await;
    log::info!("logging online; starting blinky loop");

    let mut tick: u32 = 0;

    loop {
        led0.toggle();
        tick = tick.wrapping_add(1);
        log::info!("blinky tick={}", tick);
        Timer::after(Duration::from_millis(250)).await;
    }
}
