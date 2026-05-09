use embassy_usb::Builder;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_acm::State;

use crate::hal;
use crate::interrupts::UsbFsIrqs;
use crate::parts::UsbParts;

pub type FsDriver<'d> = hal::usb::Driver<'d, hal::peripherals::USB_OTG_FS>;

pub struct UsbSerialConfig<'a> {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: Option<&'a str>,
    pub product: Option<&'a str>,
    pub serial_number: Option<&'a str>,
    pub max_packet_size: u16,
    pub vbus_detection: bool,
}

impl Default for UsbSerialConfig<'_> {
    fn default() -> Self {
        Self {
            vid: 0x1209,
            pid: 0x0001,
            manufacturer: Some("TBS"),
            product: Some("Lucid H7 USB Serial"),
            serial_number: None,
            max_packet_size: 64,
            vbus_detection: false,
        }
    }
}

pub struct UsbSerialBuffers<'d> {
    pub ep_out: &'d mut [u8],
    pub config_descriptor: &'d mut [u8],
    pub bos_descriptor: &'d mut [u8],
    pub msos_descriptor: &'d mut [u8],
    pub control: &'d mut [u8],
}

pub struct UsbSerial<'d> {
    pub class: CdcAcmClass<'d, FsDriver<'d>>,
    pub device: embassy_usb::UsbDevice<'d, FsDriver<'d>>,
}

impl<'d> UsbParts<'d> {
    pub fn into_usb_serial(
        self,
        cfg: UsbSerialConfig<'d>,
        bufs: UsbSerialBuffers<'d>,
        state: &'d mut State<'d>,
    ) -> UsbSerial<'d> {
        let mut usb_cfg = hal::usb::Config::default();
        usb_cfg.vbus_detection = cfg.vbus_detection;

        let driver = hal::usb::Driver::new_fs(
            self.otg_fs,
            UsbFsIrqs,
            self.dp,
            self.dm,
            bufs.ep_out,
            usb_cfg,
        );

        let mut device_cfg = embassy_usb::Config::new(cfg.vid, cfg.pid);
        device_cfg.manufacturer = cfg.manufacturer;
        device_cfg.product = cfg.product;
        device_cfg.serial_number = cfg.serial_number;

        let mut builder = Builder::new(
            driver,
            device_cfg,
            bufs.config_descriptor,
            bufs.bos_descriptor,
            bufs.msos_descriptor,
            bufs.control,
        );

        let class = CdcAcmClass::new(&mut builder, state, cfg.max_packet_size);
        let device = builder.build();

        UsbSerial { class, device }
    }
}
