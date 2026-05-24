#![no_std]

pub use embassy_stm32 as hal;
pub use stm32_metapac as pac;
pub use uf_bsp_core as core;

pub mod board;
pub mod clocks;
pub mod dma;
pub mod interrupts;
pub mod parts;
pub mod pins;
pub mod usb;

pub use board::Board;
pub use clocks::config;
pub use clocks::config_with_usb;
