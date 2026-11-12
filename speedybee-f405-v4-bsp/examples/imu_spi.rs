#![no_std]
#![no_main]

#[path = "support/usb_logger.rs"]
mod usb_logger;

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::spi;
use embassy_futures::yield_now;
use embassy_time::Delay;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::with_timeout;
use embedded_hal_async::spi::SpiDevice;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

bsp::bind_board_interrupts!(struct Irqs {
    primary_imu;
    primary_imu_drdy;
});

const IMU_ODR_HZ: u32 = 1_000;
const LOG_HZ: u32 = 2;
const LOG_EVERY_N_SAMPLES: u32 = IMU_ODR_HZ / LOG_HZ;
// The driver configures a one-packet watermark and 20-byte high-resolution packets.
const FIFO_PACKET_BYTES: u16 = core::mem::size_of::<icm426xx::fifo::FifoPacket4>() as u16;
const MAX_DRAIN_SAMPLES: usize = 32;
const FIFO_POLL_TIMEOUT: Duration = Duration::from_millis(10);
const ERROR_BACKOFF: Duration = Duration::from_millis(1);

async fn blink_forever(mut led: Output<'static>, period_ms: u64) -> ! {
    loop {
        led.toggle();
        Timer::after(Duration::from_millis(period_ms)).await;
    }
}

fn log_sample(sample: &icm426xx::Sample) {
    if let Some((gx, gy, gz)) = sample.gyro {
        log::info!("gyro rad/s: x={} y={} z={}", gx, gy, gz);
    }

    if let Some((ax, ay, az)) = sample.accel {
        log::info!("accel m/s^2: x={} y={} z={}", ax, ay, az);
    }
}

async fn init_imu<SPI>(
    spi_device: SPI,
) -> Result<icm426xx::ICM42688<SPI, icm426xx::Ready>, icm426xx::Error<SPI::Error>>
where
    SPI: SpiDevice,
{
    let icm = icm426xx::ICM42688::new(spi_device);
    let imu_config = icm426xx::Config {
        int1_mode: icm426xx::InterruptMode::Latched,
        int1_polarity: icm426xx::InterruptPolarity::ActiveHigh,
        rate: icm426xx::OutputDataRate::Hz1000,
        timestamps_are_absolute: false,
    };

    icm.initialize(Delay, imu_config).await
}

async fn run_imu_demo<SPI>(
    mut icm: icm426xx::ICM42688<SPI, icm426xx::Ready>,
    mut drdy: bsp::parts::ImuDrdyInput<'static>,
    mut led0: Output<'static>,
) -> !
where
    SPI: SpiDevice,
{
    let mut sample_count = 0u32;

    // Drain startup data immediately. Keep draining across bounded passes when
    // backlogged, even if reading FIFO_DATA has already cleared latched INT1.
    let mut drain_pending = true;
    let mut reset_pending = false;
    loop {
        if reset_pending {
            // A failed data transfer may have consumed part of a packet. Do not
            // parse more data until flushing has restored packet alignment.
            if icm.reset_fifo().await.is_err() {
                log::warn!("IMU FIFO reset failed; retrying");
                Timer::after(ERROR_BACKOFF).await;
                continue;
            }
            reset_pending = false;
        }
        if !drain_pending {
            // An already-high INT1 completes immediately. Timeout also checks
            // the FIFO if the interrupt is absent or its notification was cleared.
            let _wake = with_timeout(FIFO_POLL_TIMEOUT, drdy.wait_for_high()).await;
        }
        drain_pending = true;

        for _ in 0..MAX_DRAIN_SAMPLES {
            match icm.read_sample().await {
                Ok(Some((sample, has_more))) => {
                    sample_count = sample_count.wrapping_add(1);
                    if sample_count.is_multiple_of(LOG_EVERY_N_SAMPLES) {
                        log_sample(&sample);
                        led0.toggle();
                    }

                    if has_more {
                        continue;
                    }
                }
                // None can also mean an invalid packet; check for more data.
                Ok(None) => {}
                Err(icm426xx::Error::FifoOverflow) => {
                    log::warn!("IMU FIFO overflow; resetting FIFO");
                    reset_pending = true;
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
                Err(icm426xx::Error::Bus(_)) => {
                    log::error!("IMU bus error; resetting FIFO");
                    reset_pending = true;
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
                Err(icm426xx::Error::WhoAmIMismatch(whoami)) => {
                    log::error!("unexpected whoami mismatch: {}", whoami);
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
            }

            // has_more describes occupancy before the packet read. Recheck
            // before sleeping to include arrivals during that transaction.
            match icm.read_fifo_count().await {
                Ok(bytes) if bytes >= FIFO_PACKET_BYTES => {}
                Ok(_) => {
                    drain_pending = false;
                    break;
                }
                Err(_) => {
                    log::error!("IMU FIFO count read failed");
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
            }
        }

        if drain_pending {
            // Bound work per poll without waiting for another sensor interrupt.
            yield_now().await;
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config_with_usb());
    let board = bsp::Board::new(p);
    let log = usb_logger::spawn_default(&spawner, board.usb, "SpeedyBee F405 IMU SPI");
    log.wait_startup().await;

    let led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = bsp::hal::time::mhz(10);

    let imu = board.imu_primary.new_spi(Irqs, spi_cfg);

    let spi = imu.spi;
    let cs = Output::new(imu.cs, Level::High, Speed::Low);

    let spi_device = match ExclusiveDevice::new_no_delay(spi, cs) {
        Ok(spi_device) => spi_device,
        Err(_) => {
            log::error!("failed to create exclusive SPI device");
            blink_forever(led0, 500).await;
        }
    };

    let icm = match init_imu(spi_device).await {
        Ok(icm) => {
            log::info!(
                "IMU initialized at {} Hz; printing every {} samples",
                IMU_ODR_HZ,
                LOG_EVERY_N_SAMPLES
            );
            icm
        }
        Err(_) => {
            log::error!("IMU init failed");
            blink_forever(led0, 250).await;
        }
    };

    let drdy = bsp::parts::setup_primary_imu_drdy(
        imu.int,
        imu.int_exti,
        Irqs,
        Pull::None,
        interrupt::Priority::P6,
    );

    run_imu_demo(icm, drdy, led0).await
}
