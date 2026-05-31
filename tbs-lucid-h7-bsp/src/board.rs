use crate::dma::Adc1Dma;
use crate::dma::Adc3Dma;
use crate::dma::DmaResources;
use crate::dma::Usart6Dma;
use crate::hal;
use crate::parts::AdcParts;
use crate::parts::AuxParts;
use crate::parts::BaroParts;
use crate::parts::I2cParts;
use crate::parts::Leds;
use crate::parts::MotorParts;
use crate::parts::OsdParts;
use crate::parts::PrimaryImu;
use crate::parts::ReceiverParts;
use crate::parts::SdioParts;
use crate::parts::SecondaryImu;
use crate::parts::Spi3Parts;
use crate::parts::UartPortsParts;
use crate::parts::UsbParts;

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub imu_primary: PrimaryImu<'d>,
    pub imu_secondary: SecondaryImu<'d>,
    pub osd: OsdParts<'d>,
    pub spi3: Spi3Parts<'d>,
    pub receiver: ReceiverParts<'d>,
    pub uarts: UartPortsParts<'d>,
    pub i2c: I2cParts<'d>,
    pub baro: BaroParts<'d>,
    pub sdio: SdioParts<'d>,
    pub aux: AuxParts<'d>,
    pub motors: MotorParts<'d>,
    pub adc: AdcParts<'d>,
    pub dma: DmaResources<'d>,
    pub usb: UsbParts<'d>,
}

impl Board<'_> {
    pub fn new(p: hal::Peripherals) -> Self {
        Self {
            leds: Leds {
                led0: p.PE3,
                led1: p.PE4,
            },
            imu_primary: PrimaryImu {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PD7,
                cs: p.PC15,
                int: p.PB2,
                int_exti: p.EXTI2,
                dma_tx: p.DMA1_CH0,
                dma_rx: p.DMA1_CH1,
            },
            imu_secondary: SecondaryImu {
                spi: p.SPI4,
                sck: p.PE12,
                miso: p.PE13,
                mosi: p.PE14,
                cs: p.PE11,
                int: p.PE15,
                int_exti: p.EXTI15,
                dma_tx: p.DMA1_CH6,
                dma_rx: p.DMA1_CH7,
            },
            osd: OsdParts {
                spi: p.SPI2,
                sck: p.PB13,
                miso: p.PB14,
                mosi: p.PB15,
                cs: p.PB12,
                dma_tx: p.DMA1_CH2,
                dma_rx: p.DMA1_CH3,
            },
            spi3: Spi3Parts {
                spi: p.SPI3,
                sck: p.PB3,
                miso: p.PB4,
                mosi: p.PB5,
            },
            receiver: ReceiverParts {
                uart: p.USART6,
                tx: p.PC6,
                rx: p.PC7,
            },
            uarts: UartPortsParts {
                uart1: p.USART1,
                uart1_tx: p.PA9,
                uart1_rx: p.PA10,
                uart2: p.USART2,
                uart2_tx: p.PD5,
                uart2_rx: p.PD6,
                uart3: p.USART3,
                uart3_tx: p.PD8,
                uart3_rx: p.PD9,
                uart4: p.UART4,
                uart4_tx: p.PB9,
                uart4_rx: p.PB8,
                uart7: p.UART7,
                uart7_tx: p.PE8,
                uart7_rx: p.PE7,
                uart8: p.UART8,
                uart8_tx: p.PE1,
                uart8_rx: p.PE0,
            },
            i2c: I2cParts {
                i2c1: p.I2C1,
                i2c1_scl: p.PB6,
                i2c1_sda: p.PB7,
            },
            baro: BaroParts {
                i2c2: p.I2C2,
                i2c2_scl: p.PB10,
                i2c2_sda: p.PB11,
            },
            sdio: SdioParts {
                sdmmc1: p.SDMMC1,
                ck: p.PC12,
                cmd: p.PD2,
                d0: p.PC8,
                d1: p.PC9,
                d2: p.PC10,
                d3: p.PC11,
            },
            aux: AuxParts {
                servo1: p.PE5,
                servo2: p.PE6,
                led_strip: p.PA8,
                beeper: p.PA15,
                pinio1: p.PD10,
                pinio2: p.PD11,
                pinio3: p.PC13,
            },
            motors: MotorParts {
                tim3: p.TIM3,
                tim5: p.TIM5,
                m1: p.PB0,
                m2: p.PB1,
                m3: p.PA0,
                m4: p.PA1,
                m5: p.PA2,
                m6: p.PA3,
                m7: p.PD12,
                m8: p.PD13,
            },
            adc: AdcParts {
                adc1: p.ADC1,
                adc3: p.ADC3,
                vbat: p.PC0,
                current: p.PC1,
                rssi: p.PC5,
                external1: p.PC4,
                external2: p.PA4,
                external3: p.PA7,
            },
            dma: DmaResources {
                adc1: Adc1Dma { ch: p.DMA2_CH1 },
                adc3: Adc3Dma { ch: p.DMA2_CH2 },
                usart6: Usart6Dma { tx: None, rx: None },
            },
            usb: UsbParts {
                otg_fs: p.USB_OTG_FS,
                dm: p.PA11,
                dp: p.PA12,
            },
        }
    }
}
