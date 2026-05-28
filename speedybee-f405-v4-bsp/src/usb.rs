use embassy_usb::class::cdc_acm::State;

use crate::core::usb_cdc;
use crate::hal;
use crate::interrupts::UsbFsIrqs;
use crate::parts::UsbParts;

pub type FsDriver<'d> = hal::usb::Driver<'d, hal::peripherals::USB_OTG_FS>;
pub type UsbCdcAcm<'d> = usb_cdc::CdcAcm<'d, FsDriver<'d>>;
pub type UsbCdcAcmBuffers<'d> = usb_cdc::CdcAcmBuffersWithEpOut<'d>;

pub struct UsbCdcAcmConfig<'a> {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: Option<&'a str>,
    pub product: Option<&'a str>,
    pub serial_number: Option<&'a str>,
    pub max_packet_size: u16,
    pub max_power_ma: Option<u16>,
    pub vbus_detection: bool,
}

impl Default for UsbCdcAcmConfig<'_> {
    fn default() -> Self {
        Self {
            vid: 0xc0de,
            pid: 0xcafe,
            manufacturer: Some("Embassy"),
            product: Some("SpeedyBee F405 USB CDC-ACM"),
            serial_number: None,
            max_packet_size: 64,
            max_power_ma: Some(100),
            vbus_detection: false,
        }
    }
}

impl<'d> UsbParts<'d> {
    pub fn into_usb_cdc_acm(
        self,
        cfg: &UsbCdcAcmConfig<'d>,
        bufs: &'d mut UsbCdcAcmBuffers<'d>,
        state: &'d mut State<'d>,
    ) -> UsbCdcAcm<'d> {
        let descriptor_cfg = usb_cdc::CdcAcmConfig {
            vid: cfg.vid,
            pid: cfg.pid,
            manufacturer: cfg.manufacturer,
            product: cfg.product,
            serial_number: cfg.serial_number,
            max_packet_size: cfg.max_packet_size,
            max_power_ma: cfg.max_power_ma,
        };

        usb_cdc::build_usb_cdc_acm(
            |ep_out, vbus_detection| {
                let mut usb_cfg = hal::usb::Config::default();
                usb_cfg.vbus_detection = vbus_detection;
                hal::usb::Driver::new_fs(
                    self.otg_fs,
                    UsbFsIrqs,
                    self.dp,
                    self.dm,
                    ep_out,
                    usb_cfg,
                )
            },
            &descriptor_cfg,
            cfg.vbus_detection,
            bufs,
            state,
        )
    }
}
