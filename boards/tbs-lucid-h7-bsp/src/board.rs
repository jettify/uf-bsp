use crate::hal;
use crate::parts::AdcParts;
use crate::parts::ImuPrimaryParts;
use crate::parts::ImuSecondaryParts;
use crate::parts::Leds;
use crate::parts::MotorParts;
use crate::parts::ReceiverParts;
use crate::parts::UsbParts;

pub struct Board<'d> {
    pub leds: Leds<'d>,
    pub imu_primary: ImuPrimaryParts<'d>,
    pub imu_secondary: ImuSecondaryParts<'d>,
    pub receiver: ReceiverParts<'d>,
    pub motors: MotorParts<'d>,
    pub adc: AdcParts<'d>,
    pub usb: UsbParts<'d>,
}

impl<'d> Board<'d> {
    pub fn new(p: hal::Peripherals) -> Self {
        Self {
            leds: Leds {
                led0: p.PE3,
                led1: p.PE4,
            },
            imu_primary: ImuPrimaryParts {
                spi: p.SPI1,
                sck: p.PA5,
                miso: p.PA6,
                mosi: p.PD7,
                tx_dma: p.DMA1_CH3,
                rx_dma: p.DMA1_CH4,
                cs: p.PC15,
                int: p.PB2,
                int_exti: p.EXTI2,
            },
            imu_secondary: ImuSecondaryParts {
                spi: p.SPI4,
                sck: p.PE12,
                miso: p.PE13,
                mosi: p.PE14,
                tx_dma: p.DMA1_CH6,
                rx_dma: p.DMA1_CH7,
                cs: p.PE11,
                int: p.PE15,
                int_exti: p.EXTI15,
            },
            receiver: ReceiverParts {
                uart: p.USART6,
                tx: p.PC6,
                rx: p.PC7,
            },
            motors: MotorParts {
                tim3: p.TIM3,
                tim5: p.TIM5,
                m1: p.PB0,
                m2: p.PB1,
                m3: p.PA0,
                m4: p.PA1,
            },
            adc: AdcParts {
                adc1: p.ADC1,
                vbat: p.PC0,
                current: p.PC1,
                rssi: p.PC5,
            },
            usb: UsbParts {
                otg_fs: p.USB_OTG_FS,
                dm: p.PA11,
                dp: p.PA12,
            },
        }
    }
}
