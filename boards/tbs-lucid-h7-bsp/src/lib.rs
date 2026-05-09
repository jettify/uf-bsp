#![no_std]

pub use embassy_stm32 as hal;
pub use stm32_metapac as pac;

pub mod board;
pub mod clocks;
pub mod interrupts;
pub mod pins;
pub mod parts;

pub use board::Board;
pub use clocks::config;
