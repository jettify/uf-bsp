#![no_std]
#![no_main]

use bsp::hal::exti;
use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::interrupt::InterruptExt as _;
use bsp::hal::spi;
use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Delay;
use embedded_hal_bus::spi::ExclusiveDevice;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

const IMU_ODR_HZ: u32 = 1_000;
const LOG_HZ: u32 = 2;
const LOG_EVERY_N_SAMPLES: u32 = IMU_ODR_HZ / LOG_HZ;

static DRDY_LATCH_STATE: bsp::core::irq_latch::IrqLatchState =
    bsp::core::irq_latch::IrqLatchState::new();

#[bsp::hal::interrupt]
fn EXTI2() {
    bsp::pac::EXTI.pr(0).write(|w| w.set_line(2, true));
    DRDY_LATCH_STATE.on_irq();
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    info!("starting imu_spi example");
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::High, Speed::Low);

    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = bsp::hal::time::mhz(10);

    let imu = board.imu_primary.new_spi(spi_cfg);
    let cs = Output::new(imu.cs, Level::High, Speed::Low);
    let spi = imu.spi;
    let spi_device = ExclusiveDevice::new_no_delay(spi, cs).unwrap();

    let icm = icm426xx::ICM42688::new(spi_device);
    let imu_config = icm426xx::Config {
        int1_mode: icm426xx::InterruptMode::Pulsed,
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

    let drdy_input =
        exti::ExtiInput::new_blocking(imu.int, imu.int_exti, Pull::None, exti::TriggerEdge::Rising);
    let drdy = bsp::core::irq_latch::IrqLatch::new(drdy_input, &DRDY_LATCH_STATE);
    interrupt::EXTI2.set_priority(interrupt::Priority::P6);
    // SAFETY: `drdy_input` configured EXTI2 and the handler above clears the pending bit.
    unsafe { interrupt::EXTI2.enable() };
    info!("imu initialized at {} Hz; waiting for drdy", IMU_ODR_HZ);

    let mut last_drdy = drdy.current_count();
    let mut sample_count: u32 = 0;
    loop {
        drdy.wait(&mut last_drdy).await;

        loop {
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

                    if !has_more {
                        break;
                    }
                }
                Ok(None) => break,
                Err(icm426xx::Error::FifoOverflow) => {
                    info!("imu fifo overflow; resetting fifo");
                    if icm.reset_fifo().await.is_err() {
                        info!("imu fifo reset failed");
                    }
                    led1.toggle();
                    break;
                }
                Err(icm426xx::Error::Bus(_)) => {
                    info!("imu bus error");
                    led1.toggle();
                    break;
                }
                Err(icm426xx::Error::WhoAmIMismatch(whoami)) => {
                    info!("unexpected whoami mismatch: {}", whoami);
                    led1.toggle();
                    break;
                }
            }
        }
    }
}
