use crate::dma::Adc1Dma;
use crate::dma::DmaLayout;
use crate::dma::DmaResources;
use crate::dma::Spi3Dma;
use crate::dma::Usart2Dma;
use crate::hal;
use crate::parts::AdcParts;
use crate::parts::AuxParts;
use crate::parts::BaroParts;
use crate::parts::Leds;
use crate::parts::MotorParts;
use crate::parts::OsdParts;
use crate::parts::PrimaryImu;
use crate::parts::ReceiverParts;
use crate::parts::SdcardSpiParts;
use crate::parts::UartPortsParts;
use crate::parts::UsbParts;

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub imu_primary: PrimaryImu<'d>,
    pub osd: OsdParts<'d>,
    pub sdcard_spi: SdcardSpiParts<'d>,
    pub receiver: ReceiverParts<'d>,
    pub uarts: UartPortsParts<'d>,
    pub usb: UsbParts<'d>,
    pub baro: BaroParts<'d>,
    pub aux: AuxParts<'d>,
    pub motors: MotorParts<'d>,
    pub adc: AdcParts<'d>,
    pub dma: DmaResources<'d>,
}

impl Board<'_> {
    pub fn new(p: hal::Peripherals) -> Self {
        Self::new_with_dma_layout(p, DmaLayout::ImuSpiPreferred)
    }

    pub fn new_with_dma_layout(p: hal::Peripherals, layout: DmaLayout) -> Self {
        let (spi1_rx_dma, adc1_dma) = match layout {
            DmaLayout::ImuSpiPreferred => (Some(p.DMA2_CH0), None),
            DmaLayout::Adc1Preferred => (None, Some(p.DMA2_CH0)),
        };

        Self {
            leds: Leds { led0: p.PC13 },
            imu_primary: PrimaryImu {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PA7,
                cs: p.PA4,
                int: p.PC4,
                int_exti: p.EXTI4,
                dma_tx: p.DMA2_CH3,
                dma_rx: spi1_rx_dma,
            },
            osd: OsdParts {
                spi: p.SPI2,
                sck: p.PB13,
                miso: p.PC2,
                mosi: p.PC3,
                cs: p.PB12,
                dma_tx: p.DMA1_CH4,
                dma_rx: p.DMA1_CH3,
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
            uarts: UartPortsParts {
                uart1: p.USART1,
                uart1_tx: p.PA9,
                uart1_rx: p.PA10,
                uart3: p.USART3,
                uart3_tx: p.PC10,
                uart3_rx: p.PC11,
                uart4: p.UART4,
                uart4_tx: p.PA0,
                uart4_rx: p.PA1,
                uart5: p.UART5,
                uart5_rx: p.PD2,
                uart6: p.USART6,
                uart6_tx: p.PC6,
                uart6_rx: p.PC7,
            },
            usb: UsbParts {
                otg_fs: p.USB_OTG_FS,
                dm: p.PA11,
                dp: p.PA12,
            },
            baro: BaroParts {
                i2c1: p.I2C1,
                i2c1_scl: p.PB8,
                i2c1_sda: p.PB9,
            },
            aux: AuxParts {
                beeper: p.PC15,
                servo1: p.PB15,
                led_strip: p.PA8,
                camera_control: p.PB14,
                pinio1: p.PB11,
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
                spi3: Spi3Dma {
                    tx: p.DMA1_CH5,
                    rx: p.DMA1_CH0,
                },
                adc1: Adc1Dma { ch: adc1_dma },
                usart2: Usart2Dma { tx: None, rx: None },
            },
        }
    }
}
