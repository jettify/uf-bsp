//! Board-oriented pin naming for SpeedyBee F405 V4.
//!
//! Names follow flight-controller usage while keeping concrete MCU pins explicit.

use crate::hal::peripherals;

pub type Led0 = peripherals::PC13;

pub type Imu1Sck = peripherals::PA5;
pub type Imu1Miso = peripherals::PA6;
pub type Imu1Mosi = peripherals::PA7;
pub type Imu1Cs = peripherals::PA4;
pub type Imu1Int = peripherals::PC4;

pub type OsdSck = peripherals::PB13;
pub type OsdMiso = peripherals::PC2;
pub type OsdMosi = peripherals::PC3;
pub type OsdCs = peripherals::PB12;

pub type SdcardSck = peripherals::PB3;
pub type SdcardMiso = peripherals::PB4;
pub type SdcardMosi = peripherals::PB5;
pub type SdcardCs = peripherals::PC14;

pub type ReceiverTx = peripherals::PA2;
pub type ReceiverRx = peripherals::PA3;

pub type Motor1 = peripherals::PB6;
pub type Motor2 = peripherals::PB7;
pub type Motor3 = peripherals::PB0;
pub type Motor4 = peripherals::PB1;
pub type Motor5 = peripherals::PC8;
pub type Motor6 = peripherals::PC9;
pub type Motor7 = peripherals::PB10;
pub type Motor8 = peripherals::PA15;

pub type VbatAdc = peripherals::PC0;
pub type CurrentAdc = peripherals::PC1;
pub type RssiAdc = peripherals::PC5;
