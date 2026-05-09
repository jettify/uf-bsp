use crate::dma::Adc1Dma;
use crate::dma::DmaResources;
use crate::dma::Spi1Dma;
use crate::dma::Spi2Dma;
use crate::dma::Spi3Dma;
use crate::dma::Usart2Dma;
use crate::hal;
use crate::parts::AdcParts;
use crate::parts::ImuPrimaryParts;
use crate::parts::Leds;
use crate::parts::MotorParts;
use crate::parts::OsdParts;
use crate::parts::ReceiverParts;
use crate::parts::SdcardSpiParts;

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub imu_primary: ImuPrimaryParts<'d>,
    pub osd: OsdParts<'d>,
    pub sdcard_spi: SdcardSpiParts<'d>,
    pub receiver: ReceiverParts<'d>,
    pub motors: MotorParts<'d>,
    pub adc: AdcParts<'d>,
    pub dma: DmaResources<'d>,
}

impl<'d> Board<'d> {
    pub fn new(p: hal::Peripherals) -> Self {
        Self {
            leds: Leds { led0: p.PC13 },
            imu_primary: ImuPrimaryParts {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PA7,
                cs: p.PA4,
                int: p.PC4,
                int_exti: p.EXTI4,
            },
            osd: OsdParts {
                spi: p.SPI2,
                sck: p.PB13,
                miso: p.PC2,
                mosi: p.PC3,
                cs: p.PB12,
            },
            sdcard_spi: SdcardSpiParts {
                spi: p.SPI3,
                sck: p.PB3,
                miso: p.PB4,
                mosi: p.PB5,
                cs: p.PC14,
            },
            receiver: ReceiverParts {
                uart: p.USART2,
                tx: p.PA2,
                rx: p.PA3,
            },
            motors: MotorParts {
                m1: p.PB6,
                m2: p.PB7,
                m3: p.PB0,
                m4: p.PB1,
                m5: p.PC8,
                m6: p.PC9,
                m7: p.PB10,
                m8: p.PA15,
            },
            adc: AdcParts {
                adc1: p.ADC1,
                vbat: p.PC0,
                current: p.PC1,
                rssi: p.PC5,
            },
            dma: DmaResources {
                spi1: Spi1Dma {
                    tx: p.DMA2_CH3,
                    rx: p.DMA2_CH0,
                },
                spi2: Spi2Dma {
                    tx: p.DMA1_CH4,
                    rx: p.DMA1_CH3,
                },
                spi3: Spi3Dma {
                    tx: p.DMA1_CH5,
                    rx: p.DMA1_CH0,
                },
                adc1: Adc1Dma { ch: None },
                usart2: Usart2Dma { tx: None, rx: None },
            },
        }
    }
}
