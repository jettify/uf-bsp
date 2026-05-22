#![no_std]
#![no_main]

#[path = "support/usb_logger.rs"]
mod usb_logger;

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use embassy_futures::join::join3;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config_with_usb());
    let board = bsp::Board::new(p);
    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let bsp::usb::UsbCdcAcm {
        mut device,
        class: logger_class,
    } = usb_logger::init(board.usb, "SpeedyBee F405 USB Logger");
    let log_fut =
        embassy_usb_logger::with_class!(1024, log::LevelFilter::Info, logger_class);

    let blink_fut = async move {
        let mut tick: u32 = 0;

        loop {
            led0.toggle();
            tick = tick.wrapping_add(1);
            log::info!("blinky tick={}", tick);
            Timer::after(Duration::from_millis(250)).await;
        }
    };

    join3(device.run(), log_fut, blink_fut).await;
}
