# uf-bsp

[![CI](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml/badge.svg)](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml)
[![crates.io](https://img.shields.io/crates/v/uf-bsp)](https://crates.io/crates/uf-bsp)
[![docs.rs](https://img.shields.io/docsrs/uf-bsp)](https://docs.rs/uf-bsp/latest/uf_bsp/)

Board support crates for common flight controllers, boards might be usefull not only
for quadcopters and flight controllers, but also for robotics and rapid prototyping.

Originally developed for the `uflight` flight-controller project, but useful as a standalone set of board support crates.

> **Status:** This project is a work in progress. APIs, board coverage, and examples may change.

1. [`TBS Lucid H7 FC`](https://www.team-blacksheep.com/products/prod:lucid_h7) (Exposes SWDIO/SWCLK pads; can be flashed with a debug probe or via DFU.)
1. [`SpeedyBee F405 V4`](https://www.speedybee.com/speedybee-f405-v4-bls-60a-30x30-fc-esc-stack/) (Flashing is supported via DFU only.)
1. Most Betaflight/INAV-compatible boards can be added in a similar way.

## Simple Example

```rust

#![no_std]
#![no_main]

use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    info!("starting blinky example");
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut led1 = Output::new(board.leds.led1, Level::High, Speed::Low);
    info!("leds initialized");

    loop {
        led0.toggle();
        led1.toggle();
        info!("leds toggled");
        Timer::after(Duration::from_millis(250)).await;
    }
}
```


## IMU Read Example
```rust
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
    let mut icm = icm.initialize(Delay, imu_config).await.unwrap();

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
                    let _ = icm.reset_fifo().await;
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

```

## License

This project is licensed under the `Apache 2.0`. See the [LICENSE](https://github.com/jettify/uf-bsp/blob/master/LICENSE) file for details.
