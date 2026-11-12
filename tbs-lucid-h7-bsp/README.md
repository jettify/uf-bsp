# tbs-lucid-h7-bsp

[![CI](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml/badge.svg)](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml)
[![crates.io](https://img.shields.io/crates/v/tbs-lucid-h7-bsp)](https://crates.io/crates/tbs-lucid-h7-bsp)
[![docs.rs](https://docs.rs/tbs-lucid-h7-bsp/badge.svg)](https://docs.rs/tbs-lucid-h7-bsp)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/jettify/uf-bsp/blob/master/LICENSE)

Embassy-based, `no_std` board support for the **TBS Lucid H7 (STM32H743)**.
Rust 1.92+ · `thumbv7em-none-eabihf` · SWD or DFU flashing.

## Features

- Primary SPI1 and secondary SPI4 IMU resources with DMA and async DRDY inputs.
- Barometer I2C, ADC1/ADC3, and battery/current sensing pins.
- USB CDC, USART6 receiver, additional UARTs, OSD SPI2, and SDMMC resources.
- LEDs, motor output pins, board metadata, and unassigned peripherals.
- Clock presets for 480 MHz CPU and USB; Embassy time driver on TIM12.
- Optional `self-test` diagnostics and `sensor-examples` drivers.

```toml
[dependencies]
tbs-lucid-h7-bsp = "0.1.0"
```

## Examples

| Example | Purpose | Required feature |
| --- | --- | --- |
| `blinky` | LED output | — |
| `imu_spi` | IMU FIFO sampling at 1 kHz, RTT logging | `sensor-examples` |
| `baro_i2c` | Pressure and temperature readings | `sensor-examples` |
| `self_check` | Peripheral checks and USB reports | `self-test` |
| `usb_serial` | USB CDC serial | — |
| `clock_report` | Clock configuration report | — |

From the workspace root:

```sh
cargo build --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf --example imu_spi --features sensor-examples
probe-rs run --chip STM32H743VIHx target/thumbv7em-none-eabihf/release/examples/imu_spi
```

For USB diagnostics over DFU: `just self-check lucid-h7`.

[Example sources](https://github.com/jettify/uf-bsp/tree/master/tbs-lucid-h7-bsp/examples)

## License

Licensed under [Apache-2.0](https://github.com/jettify/uf-bsp/blob/master/LICENSE).
