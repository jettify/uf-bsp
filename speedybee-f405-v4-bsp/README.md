# speedybee-f405-v4-bsp

[![CI](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml/badge.svg)](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml)
[![crates.io](https://img.shields.io/crates/v/speedybee-f405-v4-bsp)](https://crates.io/crates/speedybee-f405-v4-bsp)
[![docs.rs](https://docs.rs/speedybee-f405-v4-bsp/badge.svg)](https://docs.rs/speedybee-f405-v4-bsp)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/jettify/uf-bsp/blob/master/LICENSE)

Embassy-based, `no_std` board support for the **SpeedyBee F405 V4 (STM32F405)**.
Rust 1.92+ · `thumbv7em-none-eabihf` · DFU flashing.

## Features

- SPI1 IMU resources with DMA and async DRDY input.
- Barometer I2C and ADC1 resources, with independent IMU RX and ADC DMA streams.
- USB CDC, USART2 receiver, additional UARTs, OSD SPI2, and SD card SPI3 resources.
- LED, motor output pins, board metadata, and unassigned peripherals.
- Clock presets for 168 MHz CPU and USB; Embassy time driver on TIM9.
- Optional `self-test` diagnostics and `sensor-examples` drivers.

```toml
[dependencies]
speedybee-f405-v4-bsp = "0.1.0"
```

## Examples

| Example | Purpose | Required feature |
| --- | --- | --- |
| `blinky` | LED output | — |
| `imu_spi` | IMU FIFO sampling at 1 kHz, USB logging | `sensor-examples` |
| `baro_i2c` | Pressure and temperature readings | `sensor-examples` |
| `self_check` | Peripheral checks and USB reports | `self-test` |

From the workspace root:

```sh
cargo build --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf --example imu_spi --features sensor-examples
```

For USB diagnostics over DFU: `just self-check speedybee-f405-v4`.

[Example sources](https://github.com/jettify/uf-bsp/tree/master/speedybee-f405-v4-bsp/examples)

## License

Licensed under [Apache-2.0](https://github.com/jettify/uf-bsp/blob/master/LICENSE).
