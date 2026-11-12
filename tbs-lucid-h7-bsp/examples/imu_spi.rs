#![no_std]
#![no_main]

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::spi;
use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_futures::yield_now;
use embassy_time::Delay;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::with_timeout;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

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

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    info!("starting imu_spi example");
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::High, Speed::Low);

    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = bsp::hal::time::mhz(10);

    let imu = board.imu_primary.new_spi(Irqs, spi_cfg);
    let cs = Output::new(imu.cs, Level::High, Speed::Low);
    let spi = imu.spi;
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let icm = icm426xx::ICM42688::new(spi_device);
    let imu_config = icm426xx::Config {
        int1_mode: icm426xx::InterruptMode::Latched,
        int1_polarity: icm426xx::InterruptPolarity::ActiveHigh,
        rate: icm426xx::OutputDataRate::Hz1000,
        timestamps_are_absolute: false,
    };
    let mut icm = match icm.initialize(Delay, imu_config).await {
        Ok(icm) => icm,
        Err(_) => {
            info!("imu init failed");
            loop {
                led1.toggle();
                embassy_time::Timer::after_millis(250).await;
            }
        }
    };

    let mut drdy = bsp::parts::setup_primary_imu_drdy(
        imu.int,
        imu.int_exti,
        Irqs,
        Pull::None,
        interrupt::Priority::P6,
    );
    info!("imu initialized at {} Hz; waiting for drdy", IMU_ODR_HZ);

    let mut sample_count: u32 = 0;
    // Drain startup data immediately. Keep draining across bounded passes when
    // backlogged, even if reading FIFO_DATA has already cleared latched INT1.
    let mut drain_pending = true;
    let mut reset_pending = false;
    loop {
        if reset_pending {
            // A failed data transfer may have consumed part of a packet. Do not
            // parse more data until flushing has restored packet alignment.
            if icm.reset_fifo().await.is_err() {
                info!("imu fifo reset failed; retrying");
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
                        if let Some((gx, gy, gz)) = sample.gyro {
                            info!("gyro rad/s: x={} y={} z={}", gx, gy, gz);
                        }
                        if let Some((ax, ay, az)) = sample.accel {
                            info!("accel m/s^2: x={} y={} z={}", ax, ay, az);
                        }
                        led0.toggle();
                    }

                    if has_more {
                        continue;
                    }
                }
                // None can also mean an invalid packet; check for more data.
                Ok(None) => {}
                Err(icm426xx::Error::FifoOverflow) => {
                    info!("imu fifo overflow; resetting fifo");
                    reset_pending = true;
                    led1.toggle();
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
                Err(icm426xx::Error::Bus(_)) => {
                    info!("imu bus error; resetting fifo");
                    reset_pending = true;
                    led1.toggle();
                    Timer::after(ERROR_BACKOFF).await;
                    break;
                }
                Err(icm426xx::Error::WhoAmIMismatch(whoami)) => {
                    info!("unexpected whoami mismatch: {}", whoami);
                    led1.toggle();
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
                    info!("imu FIFO count read failed");
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
