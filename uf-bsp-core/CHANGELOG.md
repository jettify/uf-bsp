# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/jettify/uf-bsp/releases/tag/uf-bsp-core-v0.1.0) - 2026-11-13

### Added

- Rework and simplify self test.
- Remove irq latch in favor more embassy fifo approach to read from imu ([#7](https://github.com/jettify/uf-bsp/issues/7))
- Setup publish pipeline ([#4](https://github.com/jettify/uf-bsp/issues/4))
- Add self check logic to test new and existing boards.
- Add initial version of capabiliriess.
- Make sure irq latch can be claimed only once.
- Rework interrupt handling.
- Move more usb logic into core.
- Common crate.

### Other

- Add note about stability.
- Prepare packages and docs for release.
