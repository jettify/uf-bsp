# uf-bsp-core

[![CI](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml/badge.svg)](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml)
[![crates.io](https://img.shields.io/crates/v/uf-bsp-core)](https://crates.io/crates/uf-bsp-core)
[![docs.rs](https://docs.rs/uf-bsp-core/badge.svg)](https://docs.rs/uf-bsp-core)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/jettify/uf-bsp/blob/master/LICENSE)

Shared `no_std` utilities for uf-bsp board support crates. Requires Rust 1.92+.

## Features

- Board capability metadata and peripheral resource types.
- USB CDC configuration helpers.

```toml
[dependencies]
uf-bsp-core = "0.1.0"
```

## Examples

See the board `self_check` examples for diagnostics and USB reporting:

- [TBS Lucid H7](https://github.com/jettify/uf-bsp/blob/master/tbs-lucid-h7-bsp/examples/self_check.rs)
- [SpeedyBee F405 V4](https://github.com/jettify/uf-bsp/blob/master/speedybee-f405-v4-bsp/examples/self_check.rs)

[Board crates](https://github.com/jettify/uf-bsp)

## License

Licensed under [Apache-2.0](https://github.com/jettify/uf-bsp/blob/master/LICENSE).
