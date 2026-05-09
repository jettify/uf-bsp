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
pub type RxPpm = peripherals::PA3;

pub type Uart1Tx = peripherals::PA9;
pub type Uart1Rx = peripherals::PA10;
pub type Uart3Tx = peripherals::PC10;
pub type Uart3Rx = peripherals::PC11;
pub type Uart4Tx = peripherals::PA0;
pub type Uart4Rx = peripherals::PA1;
pub type Uart5Rx = peripherals::PD2;
pub type Uart6Tx = peripherals::PC6;
pub type Uart6Rx = peripherals::PC7;

pub type I2c1Scl = peripherals::PB8;
pub type I2c1Sda = peripherals::PB9;

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

pub type Beeper = peripherals::PC15;
pub type Servo1 = peripherals::PB15;
pub type LedStrip = peripherals::PA8;
pub type CameraControl = peripherals::PB14;
pub type Pinio1 = peripherals::PB11;
