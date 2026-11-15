//! Exercise the BSP async barometer bus without a sensor driver.
#![no_std]
#![no_main]

use bsp::hal::i2c::Config as I2cConfig;
use bsp::hal::time::Hertz;
use cortex_m as _;
use defmt::Debug2Format;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

bsp::bind_board_interrupts!(struct Irqs {
    baro;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);
    let mut config = I2cConfig::default();
    config.frequency = Hertz::khz(400);
    let mut i2c = board.baro.new_i2c(Irqs, config);

    loop {
        // DPS3xx product/revision ID at the primary I2C address.
        let mut id = [0u8; 1];
        match i2c.write_read(0x76, &[0x0d], &mut id).await {
            Ok(()) => info!("barometer product/revision ID: {=[u8]:x}", id),
            Err(error) => info!("barometer I2C error: {:?}", Debug2Format(&error)),
        }
        Timer::after_millis(1_000).await;
    }
}
