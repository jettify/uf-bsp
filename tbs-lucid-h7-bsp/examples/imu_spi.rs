#![no_std]
#![no_main]

use bsp::hal::bind_interrupts;
use bsp::hal::dma;
use bsp::hal::exti;
use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::peripherals;
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

bind_interrupts!(struct Irqs {
    DMA1_STREAM0 => dma::InterruptHandler<peripherals::DMA1_CH0>;
    DMA1_STREAM1 => dma::InterruptHandler<peripherals::DMA1_CH1>;
    EXTI2 => exti::InterruptHandler<interrupt::typelevel::EXTI2>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    info!("starting imu_spi example");
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::High, Speed::Low);

    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = bsp::hal::time::mhz(10);

    let spi = spi::Spi::new(
        board.imu_primary.spi,
        board.imu_primary.sck,
        board.imu_primary.mosi,
        board.imu_primary.miso,
        board.dma.spi1.tx,
        board.dma.spi1.rx,
        Irqs,
        spi_cfg,
    );
    let cs = Output::new(board.imu_primary.cs, Level::High, Speed::Low);
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

    let mut drdy = exti::ExtiInput::new(
        board.imu_primary.int,
        board.imu_primary.int_exti,
        Pull::None,
        Irqs,
    );
    drdy.wait_for_low().await;
    info!("imu initialized at {} Hz; waiting for drdy", IMU_ODR_HZ);

    let mut sample_count: u32 = 0;
    loop {
        drdy.wait_for_rising_edge().await;

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
