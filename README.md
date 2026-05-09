# uf-bsp

[![CI](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml/badge.svg)](https://github.com/jettify/uf-bsp/actions/workflows/CI.yml)
[![codecov](https://codecov.io/gh/jettify/uf-bsp/graph/badge.svg?token=NFUQBCTUXF)](https://codecov.io/gh/jettify/uf-bsp)
[![crates.io](https://img.shields.io/crates/v/uf-bsp)](https://crates.io/crates/uf-bsp)
[![docs.rs](https://img.shields.io/docsrs/uf-bsp)](https://docs.rs/uf-bsp/latest/uf_bsp/)

Board support crates for:
1. [`TBS Lucid H7 FC`](https://www.team-blacksheep.com/products/prod:lucid_h7)
1. [`SpeedyBee F405 V4`](https://www.speedybee.com/speedybee-f405-v4-bls-60a-30x30-fc-esc-stack/)

## Example Commands

```bash
cargo build --release -p tbs-lucid-h7-bsp --target thumbv7em-none-eabihf
cargo build --release -p speedybee-f405-v4-bsp --target thumbv7em-none-eabihf
```

## License

This project is licensed under the `Apache 2.0`. See the [LICENSE](https://github.com/jettify/uf-bsp/blob/master/LICENSE) file for details.
