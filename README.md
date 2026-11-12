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

## Hardware self-check

Flash a driver-free diagnostic example over DFU with `just self-check lucid-h7` or
`just self-check speedybee-f405-v4`. Results repeat over USB serial. See
[the self-check guide](docs/self-check.md) for setup, reusable core/BSP test modules,
and result interpretation.

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


## Interrupt bindings

The application owns interrupt vectors. Depending on a BSP does not emit its
peripheral interrupt handlers. Select the board peripherals you use:

```rust
bsp::bind_board_interrupts!(struct Irqs {
    primary_imu;
    usb_fs;
    receiver;
});

let imu = board.imu_primary.new_spi(Irqs, spi_config);
let usb = board.usb.into_usb_cdc_acm(Irqs, &usb_config, buffers, state);
```

Both boards support `primary_imu`, `osd`, `receiver`, and `usb_fs`.
Lucid H7 also supports `secondary_imu`. Both boards support `adc1` for their
default ADC DMA binding. Select `primary_imu_drdy` for the primary IMU's standard
Embassy EXTI binding, or `secondary_imu_drdy` for Lucid's secondary IMU.
Executor interrupts are configured separately by the application.

DRDY helpers return standard async EXTI inputs. Match the wait to the sensor
configuration: use `drdy.wait_for_high().await` for active-high latched interrupts
or `drdy.wait_for_rising_edge().await` for pulses. These inputs support other
Embassy EXTI inputs, including lines sharing the secondary IMU's vector. Events
between waits are not counted; drain the sensor FIFO to obtain all samples.
See [EXTI handling and migration](docs/exti.md) for the API change and regression checks.

For custom mappings or handlers, omit the corresponding selection and use
`bsp::hal::bind_interrupts!` directly. Constructors accept either macro's binding
types through the same compile-time bounds. Additional vectors use a separate
Embassy invocation with its own binding type, as shown below. Define each vector
once; any handlers sharing a vector belong in the same entry. Generated binding
structs are `Copy`, so they can be passed to multiple constructors.

See each board's `examples/interrupt_bindings.rs` for a complete selection example,
and each board's `examples/imu_spi.rs` for latched FIFO acquisition.

## SpeedyBee IMU and ADC DMA

`Board::new` provides SPI1 RX on `DMA2_CH2`, SPI1 TX on `DMA2_CH3`,
and ADC1 on `DMA2_CH0`. Both IMU SPI and ADC DMA are available together.
The `primary_imu` interrupt group binds `DMA2_STREAM2` and `DMA2_STREAM3`;
applications using ADC DMA bind `DMA2_STREAM0` separately, as demonstrated
in `speedybee-f405-v4-bsp/examples/resource_access.rs`.

Migration: replace `Board::new_with_dma_layout` with `Board::new`, remove
`DmaLayout` and optional IMU/ADC DMA handling, and use `new_spi`'s returned
`PrimaryImuSpi` directly instead of matching on `NewSpiError`. Custom IMU
bindings must move RX from `DMA2_STREAM0` / `DMA2_CH0` to
`DMA2_STREAM2` / `DMA2_CH2`. The former `unassigned.dma2_ch2` token now
belongs to `imu_primary.dma_rx`.

## IMU Read Example

Both boards provide an `imu_spi` example using the ICM-42688 FIFO at 1 kHz:

- [Lucid H7](tbs-lucid-h7-bsp/examples/imu_spi.rs): RTT logging.
- [SpeedyBee F405 V4](speedybee-f405-v4-bsp/examples/imu_spi.rs): USB logging.

The examples configure a latched, active-high FIFO watermark interrupt and wait
with `wait_for_high()`. They drain queued packets, recheck occupancy before
sleeping, and yield after each bounded pass. A timeout provides a fallback FIFO
check if no interrupt arrives. See [FIFO acquisition](docs/exti.md#fifo-acquisition)
for recovery behavior and hardware validation.

## License

This project is licensed under the `Apache 2.0`. See the [LICENSE](https://github.com/jettify/uf-bsp/blob/master/LICENSE) file for details.
