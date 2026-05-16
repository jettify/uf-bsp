use embassy_usb::Builder;
use embassy_usb::Config;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_acm::State;
use static_cell::StaticCell;

use crate::hal;
use crate::interrupts::UsbFsIrqs;

pub type FsDriver<'d> = hal::usb::Driver<'d, hal::peripherals::USB_OTG_FS>;

pub struct UsbLoggerParts<'d> {
    pub device: UsbDevice<'d, FsDriver<'d>>,
    pub logger_class: CdcAcmClass<'d, FsDriver<'d>>,
}

pub fn init(
    otg_fs: hal::Peri<'static, hal::peripherals::USB_OTG_FS>,
    dp: hal::Peri<'static, hal::peripherals::PA12>,
    dm: hal::Peri<'static, hal::peripherals::PA11>,
    product: &'static str,
) -> UsbLoggerParts<'static> {
    static EP_OUT: StaticCell<[u8; 256]> = StaticCell::new();
    static STATE: StaticCell<State<'static>> = StaticCell::new();
    static CONFIG_DESCRIPTOR: StaticCell<[u8; 128]> = StaticCell::new();
    static BOS_DESCRIPTOR: StaticCell<[u8; 16]> = StaticCell::new();
    static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

    let driver = hal::usb::Driver::new_fs(
        otg_fs,
        UsbFsIrqs,
        dp,
        dm,
        EP_OUT.init([0; 256]),
        hal::usb::Config::default(),
    );

    let state = STATE.init(State::new());
    let mut usb_config = Config::new(0xc0de, 0xcafe);
    usb_config.manufacturer = Some("Embassy");
    usb_config.product = Some(product);
    usb_config.serial_number = None;
    usb_config.max_power = 100;
    usb_config.max_packet_size_0 = embassy_usb_logger::MAX_PACKET_SIZE;

    let mut builder = Builder::new(
        driver,
        usb_config,
        CONFIG_DESCRIPTOR.init([0; 128]),
        BOS_DESCRIPTOR.init([0; 16]),
        MSOS_DESCRIPTOR.init([0; 256]),
        CONTROL_BUF.init([0; 64]),
    );
    let logger_class = CdcAcmClass::new(
        &mut builder,
        state,
        embassy_usb_logger::MAX_PACKET_SIZE as u16,
    );
    let device = builder.build();

    UsbLoggerParts {
        device,
        logger_class,
    }
}
