#![no_std]
#![no_main]

#[path = "support/usb_logger.rs"]
mod usb_logger;

use bsp::hal::i2c::Config as I2cConfig;
use bsp::hal::i2c::I2c;
use bsp::hal::i2c::Master;
use bsp::hal::mode::Blocking;
use bsp::hal::time::Hertz;
use embassy_time::Timer;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;
use uf_dps3xx::Config as DpsConfig;
use uf_dps3xx::Dps3xx;
use uf_dps3xx::Error as DpsError;
use uf_dps3xx::I2cAddress;
use uf_dps3xx::Poll;
use uf_dps3xx::Ready;
use uf_dps3xx::TemperatureSource;
use uf_dps3xx::Uninit;

type BoardI2c<'d> = I2c<'d, Blocking, Master>;
type BoardDps<'d> = Dps3xx<BoardI2c<'d>, Ready>;
type BoardDpsError = DpsError<bsp::hal::i2c::Error>;

enum AppError {
    Init,
    Start,
}

async fn init_dps<'d>(i2c: BoardI2c<'d>, config: DpsConfig) -> Result<BoardDps<'d>, BoardDpsError> {
    let sensor = Dps3xx::<_, Uninit>::new_i2c(i2c, I2cAddress::Primary, config)?;
    let mut init = sensor.init().map_err(|e| e.into_error())?;

    loop {
        match init.poll()? {
            Poll::Pending { wait_ms } => Timer::after_millis(u64::from(wait_ms)).await,
            Poll::Ready(sensor) => return Ok(sensor),
        }
    }
}

async fn run(baro: bsp::parts::BaroParts<'static>) -> Result<(), AppError> {
    let mut cfg = I2cConfig::default();
    cfg.frequency = Hertz::khz(400);

    let i2c = I2c::new_blocking(baro.i2c1, baro.i2c1_scl, baro.i2c1_sda, cfg);

    let dps_config = DpsConfig::default().temperature_source(TemperatureSource::External);
    let mut dps = init_dps(i2c, dps_config).await.map_err(|err| {
        log::error!("dps3xx init failed: {:?}", err);
        AppError::Init
    })?;
    dps.start_background().map_err(|err| {
        log::error!("dps3xx background start failed: {:?}", err);
        AppError::Start
    })?;

    log::info!("dps3xx initialized on I2C1");

    loop {
        match dps.try_read_sample() {
            Ok(Some(sample)) => {
                log::info!(
                    "pressure_pa={} temp_c={}",
                    sample.pressure_pa,
                    sample.temperature_c
                );
            }
            Ok(None) => {}
            Err(err) => log::error!("baro sample read error: {:?}", err),
        }
        Timer::after_millis(u64::from(dps.config().suggested_poll_period_ms())).await;
    }
}

async fn fatal(error: AppError) -> ! {
    match error {
        AppError::Init => log::error!("baro_i2c failed during init"),
        AppError::Start => log::error!("baro_i2c failed during background start"),
    }

    loop {
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config_with_usb());
    let board = bsp::Board::new(p);
    let log = usb_logger::spawn_default(&spawner, board.usb, "SpeedyBee F405 Baro I2C");
    log.wait_startup().await;
    log::info!("logging online; starting baro_i2c");

    if let Err(error) = run(board.baro).await {
        fatal(error).await;
    }
}
