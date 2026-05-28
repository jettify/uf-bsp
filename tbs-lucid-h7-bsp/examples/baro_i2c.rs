#![no_std]
#![no_main]

use bsp::hal::i2c::Config as I2cConfig;
use bsp::hal::i2c::I2c;
use bsp::hal::i2c::Master;
use bsp::hal::mode::Blocking;
use bsp::hal::time::Hertz;
use cortex_m as _;
use defmt::Debug2Format;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Timer;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;
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

async fn run() -> Result<(), AppError> {
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut cfg = I2cConfig::default();
    cfg.frequency = Hertz::khz(400);

    let i2c = board.baro.new_i2c_blocking(cfg);

    let dps_config = DpsConfig::default().temperature_source(TemperatureSource::External);
    let mut dps = init_dps(i2c, dps_config).await.map_err(|err| {
        info!("dps3xx init failed: {:?}", Debug2Format(&err));
        AppError::Init
    })?;
    dps.start_background().map_err(|err| {
        info!("dps3xx background start failed: {:?}", Debug2Format(&err));
        AppError::Start
    })?;

    info!("dps3xx initialized on I2C2");

    loop {
        match dps.try_read_sample() {
            Ok(Some(s)) => info!("pressure_pa={} temp_c={}", s.pressure_pa, s.temperature_c),
            Ok(None) => (),
            Err(_) => info!("baro read error"),
        }
        Timer::after_millis(u64::from(dps.config().suggested_poll_period_ms())).await;
    }
}

async fn fatal(error: AppError) -> ! {
    match error {
        AppError::Init => info!("baro_i2c failed during init"),
        AppError::Start => info!("baro_i2c failed during background start"),
    }
    loop {
        Timer::after_millis(250).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    if let Err(error) = run().await {
        fatal(error).await;
    }
}
