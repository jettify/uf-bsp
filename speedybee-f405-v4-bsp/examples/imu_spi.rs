#![no_std]
#![no_main]

#[path = "support/usb_logger.rs"]
mod usb_logger;

use bsp::hal::exti;
use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::interrupt::InterruptExt as _;
use bsp::hal::spi;
use embassy_time::Delay;
use embassy_time::Duration;
use embassy_time::Timer;
use embedded_hal_async::spi::SpiDevice;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

const IMU_ODR_HZ: u32 = 1_000;
const LOG_HZ: u32 = 2;
const LOG_EVERY_N_SAMPLES: u32 = IMU_ODR_HZ / LOG_HZ;

static DRDY_LATCH_STATE: bsp::core::irq_latch::IrqLatchState =
    bsp::core::irq_latch::IrqLatchState::new();

#[bsp::hal::interrupt]
fn EXTI4() {
    bsp::pac::EXTI.pr(0).write(|w| w.set_line(4, true));
    DRDY_LATCH_STATE.on_irq();
}

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
        int1_mode: icm426xx::InterruptMode::Pulsed,
        int1_polarity: icm426xx::InterruptPolarity::ActiveHigh,
        rate: icm426xx::OutputDataRate::Hz1000,
        timestamps_are_absolute: false,
    };

    icm.initialize(Delay, imu_config).await
}

async fn run_imu_demo<SPI, DRDY>(
    mut icm: icm426xx::ICM42688<SPI, icm426xx::Ready>,
    drdy: bsp::core::irq_latch::IrqLatch<DRDY>,
    mut led0: Output<'static>,
) -> !
where
    SPI: SpiDevice,
    DRDY: 'static,
{
    let mut last_drdy = drdy.current_count();
    let mut sample_count = 0u32;

    loop {
        drdy.wait(&mut last_drdy).await;

        loop {
            match icm.read_sample().await {
                Ok(Some((sample, has_more))) => {
                    sample_count = sample_count.wrapping_add(1);
                    if sample_count.is_multiple_of(LOG_EVERY_N_SAMPLES) {
                        log_sample(&sample);
                        led0.toggle();
                    }

                    if !has_more {
                        break;
                    }
                }
                Ok(None) => break,
                Err(icm426xx::Error::FifoOverflow) => {
                    log::warn!("IMU FIFO overflow; resetting FIFO");
                    if icm.reset_fifo().await.is_err() {
                        log::warn!("IMU FIFO reset failed");
                    }
                    break;
                }
                Err(icm426xx::Error::Bus(_)) => {
                    log::error!("IMU bus error");
                    break;
                }
                Err(icm426xx::Error::WhoAmIMismatch(whoami)) => {
                    log::error!("unexpected whoami mismatch: {}", whoami);
                    break;
                }
            }
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

    let imu = match board.imu_primary.new_spi(spi_cfg) {
        Ok(imu) => imu,
        Err(bsp::parts::NewSpiError::RxDmaUnavailable) => {
            log::error!("SPI1 RX DMA unavailable");
            blink_forever(led0, 250).await;
        }
    };

    let spi = imu.spi;
    let cs = Output::new(imu.cs, Level::High, Speed::Low);
    let drdy_input =
        exti::ExtiInput::new_blocking(imu.int, imu.int_exti, Pull::None, exti::TriggerEdge::Rising);

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

    let drdy = bsp::core::irq_latch::IrqLatch::new(drdy_input, &DRDY_LATCH_STATE);
    interrupt::EXTI4.set_priority(interrupt::Priority::P6);
    // SAFETY: `drdy_input` configured EXTI4 and the handler above clears the pending bit.
    unsafe { interrupt::EXTI4.enable() };

    run_imu_demo(icm, drdy, led0).await
}
