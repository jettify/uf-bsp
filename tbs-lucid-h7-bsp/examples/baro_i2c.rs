#![no_std]
#![no_main]

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use bsp::hal::i2c::Config as I2cConfig;
use bsp::hal::i2c::I2c;
use bsp::hal::i2c::Master;
use bsp::hal::mode::Blocking;
use bsp::hal::time::Hertz;
use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;
use uf_dps3xx::Config as DpsConfig;
use uf_dps3xx::Dps3xx;
use uf_dps3xx::I2cAddress;
use uf_dps3xx::Poll;
use uf_dps3xx::Uninit;

type BoardI2c<'d> = I2c<'d, Blocking, Master>;

async fn init_dps<'d>(
    i2c: BoardI2c<'d>,
    config: DpsConfig,
) -> Result<uf_dps3xx::Dps3xx<BoardI2c<'d>, uf_dps3xx::Ready>, uf_dps3xx::Error<bsp::hal::i2c::Error>>
{
    let sensor = Dps3xx::<_, Uninit>::new_i2c(i2c, I2cAddress::Primary, config)?;
    let mut init = sensor.init().map_err(|e| e.into_error())?;

    loop {
        match init.poll()? {
            Poll::Pending { wait_ms } => Timer::after_millis(u64::from(wait_ms)).await,
            Poll::Ready(sensor) => return Ok(sensor),
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::High, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::Low, Speed::Low);

    let mut i2c_cfg = I2cConfig::default();
    i2c_cfg.frequency = Hertz::khz(400);
    let i2c = I2c::new_blocking(
        board.baro.i2c2,
        board.baro.i2c2_scl,
        board.baro.i2c2_sda,
        i2c_cfg,
    );

    let dps_config = DpsConfig::default();
    let mut dps = init_dps(i2c, dps_config).await.unwrap();
    dps.start_background().unwrap();

    info!("dps3xx initialized on I2C2");

    loop {
        match dps.try_read_sample() {
            Ok(Some(sample)) => {
                info!(
                    "baro pressure_pa={} temp_c={}",
                    sample.pressure_pa, sample.temperature_c
                );
                led0.toggle();
            }
            Ok(None) => {}
            Err(_) => {
                info!("baro read error");
                led1.toggle();
            }
        }
        Timer::after_millis(u64::from(dps.config().suggested_poll_period_ms())).await;
    }
}
