//! Reusable interrupt selections for in-scope TBS Lucid H7 peripherals.

use crate::hal::interrupt::Interrupt;

pub const IMU1_SPI_IRQ: Interrupt = Interrupt::SPI1;
pub const IMU2_SPI_IRQ: Interrupt = Interrupt::SPI4;
pub const RECEIVER_UART_IRQ: Interrupt = Interrupt::USART6;
pub const MOTORS_TIM3_IRQ: Interrupt = Interrupt::TIM3;
pub const MOTORS_TIM5_IRQ: Interrupt = Interrupt::TIM5;
