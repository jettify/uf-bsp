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

pub type Uart1Tx = peripherals::PA9;
pub type Uart1Rx = peripherals::PA10;
pub type Uart2Tx = peripherals::PD5;
pub type Uart2Rx = peripherals::PD6;
pub type Uart3Tx = peripherals::PD8;
pub type Uart3Rx = peripherals::PD9;
pub type Uart4Tx = peripherals::PB9;
pub type Uart4Rx = peripherals::PB8;
pub type Uart7Tx = peripherals::PE8;
pub type Uart7Rx = peripherals::PE7;
pub type Uart8Tx = peripherals::PE1;
pub type Uart8Rx = peripherals::PE0;

pub type Motor1 = peripherals::PB0;
pub type Motor2 = peripherals::PB1;
pub type Motor3 = peripherals::PA0;
pub type Motor4 = peripherals::PA1;
pub type Motor5 = peripherals::PA2;
pub type Motor6 = peripherals::PA3;
pub type Motor7 = peripherals::PD12;
pub type Motor8 = peripherals::PD13;

pub type OsdSck = peripherals::PB13;
pub type OsdMiso = peripherals::PB14;
pub type OsdMosi = peripherals::PB15;
pub type OsdCs = peripherals::PB12;

pub type Spi3Sck = peripherals::PB3;
pub type Spi3Miso = peripherals::PB4;
pub type Spi3Mosi = peripherals::PB5;

pub type VbatAdc = peripherals::PC0;
pub type CurrentAdc = peripherals::PC1;
pub type RssiAdc = peripherals::PC5;
pub type External1Adc = peripherals::PC4;
pub type External2Adc = peripherals::PA4;
pub type External3Adc = peripherals::PA7;

pub type UsbDm = peripherals::PA11;
pub type UsbDp = peripherals::PA12;

pub type I2c1Scl = peripherals::PB6;
pub type I2c1Sda = peripherals::PB7;
pub type I2c2Scl = peripherals::PB10;
pub type I2c2Sda = peripherals::PB11;

pub type SdioCk = peripherals::PC12;
pub type SdioCmd = peripherals::PD2;
pub type SdioD0 = peripherals::PC8;
pub type SdioD1 = peripherals::PC9;
pub type SdioD2 = peripherals::PC10;
pub type SdioD3 = peripherals::PC11;

pub type Servo1 = peripherals::PE5;
pub type Servo2 = peripherals::PE6;
pub type LedStrip = peripherals::PA8;
pub type Beeper = peripherals::PA15;
pub type RxPpm = peripherals::PC7;

pub type Pinio1 = peripherals::PD10;
pub type Pinio2 = peripherals::PD11;
pub type Pinio3 = peripherals::PC13;
