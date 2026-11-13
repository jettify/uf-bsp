# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/jettify/uf-bsp/releases/tag/tbs-lucid-h7-bsp-v0.1.0) - 2026-11-13

### Added

- Rework and simplify self test.
- Remove irq latch in favor more embassy fifo approach to read from imu ([#7](https://github.com/jettify/uf-bsp/issues/7))
- Setup publish pipeline ([#4](https://github.com/jettify/uf-bsp/issues/4))
- Add self check logic to test new and existing boards.
- Add initial version of capabiliriess.
- Make sure irq latch can be claimed only once.
- Rework interrupt handling.
- User controlledd bind interrupt. ([#3](https://github.com/jettify/uf-bsp/issues/3))
- Expose unassigned peripherals. ([#2](https://github.com/jettify/uf-bsp/issues/2))
- Rework SPI setup for OSD chip.
- Add high level constructor for baro.
- Rework whole IMU API, make it simpler.
- Move more usb logic into core.
- Align USB implementation between boards.
- Add speedybee baro example.
- Reworked baro example for TBS.
- Update TBS bsp with new core crate.
- Add usb serial example.
- Apply lints on examples as well.
- Add new clock and clocks report example.
- Add barometer example.
- Allocate i2c pins for tbs barometer.
- Hardware veify first tbs example.
- Reuse worspace parameters more.

### Fixed

- Apply categories properly.
- Fix usb clocks for TBS board.

### Other

- Add note about stability.
- Add board images.
- Prepare packages and docs for release.
- Update dependencies to new version.
- Address more pedantic lints.
- More redable folder structure.
