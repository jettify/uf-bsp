//! Board-oriented pin naming for TBS Lucid H7.
//!
//! Names follow flight-controller usage while keeping concrete MCU pins explicit.

use crate::hal::peripherals;

pub type Led0 = peripherals::PE3;
pub type Led1 = peripherals::PE4;

pub type Imu1Sck = peripherals::PA5;
pub type Imu1Miso = peripherals::PA6;
pub type Imu1Mosi = peripherals::PD7;
pub type Imu1Cs = peripherals::PC15;
pub type Imu1Int = peripherals::PB2;

pub type Imu2Sck = peripherals::PE12;
pub type Imu2Miso = peripherals::PE13;
pub type Imu2Mosi = peripherals::PE14;
pub type Imu2Cs = peripherals::PE11;
pub type Imu2Int = peripherals::PE15;

pub type ReceiverTx = peripherals::PC6;
pub type ReceiverRx = peripherals::PC7;

pub type Motor1 = peripherals::PB0;
pub type Motor2 = peripherals::PB1;
pub type Motor3 = peripherals::PA0;
pub type Motor4 = peripherals::PA1;

pub type VbatAdc = peripherals::PC0;
pub type CurrentAdc = peripherals::PC1;
pub type RssiAdc = peripherals::PC5;
