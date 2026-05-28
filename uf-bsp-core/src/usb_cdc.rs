use embassy_usb::Builder;
use embassy_usb::UsbDevice;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_acm::State;
use embassy_usb::driver::Driver;

pub struct CdcAcmConfig<'a> {
    pub vid: u16,
    pub pid: u16,
    pub manufacturer: Option<&'a str>,
    pub product: Option<&'a str>,
    pub serial_number: Option<&'a str>,
    pub max_packet_size: u16,
    pub max_power_ma: Option<u16>,
}

impl Default for CdcAcmConfig<'_> {
    fn default() -> Self {
        Self {
            vid: 0xc0de,
            pid: 0xcafe,
            manufacturer: None,
            product: None,
            serial_number: None,
            max_packet_size: 64,
            max_power_ma: Some(100),
        }
    }
}

pub struct CdcAcmBuffers<'d> {
    pub config_descriptor: &'d mut [u8],
    pub bos_descriptor: &'d mut [u8],
    pub msos_descriptor: &'d mut [u8],
    pub control: &'d mut [u8],
}

pub struct CdcAcmBuffersWithEpOut<'d> {
    pub ep_out: &'d mut [u8],
    pub config_descriptor: &'d mut [u8],
    pub bos_descriptor: &'d mut [u8],
    pub msos_descriptor: &'d mut [u8],
    pub control: &'d mut [u8],
}

pub struct CdcAcm<'d, D: Driver<'d>> {
    pub class: CdcAcmClass<'d, D>,
    pub device: UsbDevice<'d, D>,
}

pub fn build_cdc_acm<'d, D: Driver<'d>>(
    driver: D,
    cfg: &CdcAcmConfig<'d>,
    bufs: CdcAcmBuffers<'d>,
    state: &'d mut State<'d>,
) -> CdcAcm<'d, D> {
    let mut device_cfg = embassy_usb::Config::new(cfg.vid, cfg.pid);
    device_cfg.manufacturer = cfg.manufacturer;
    device_cfg.product = cfg.product;
    device_cfg.serial_number = cfg.serial_number;
    device_cfg.max_packet_size_0 = cfg.max_packet_size as u8;
    if let Some(max_power_ma) = cfg.max_power_ma {
        device_cfg.max_power = max_power_ma;
    }

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

    CdcAcm { class, device }
}

pub fn build_usb_cdc_acm<'d, D, MakeDriver>(
    make_driver: MakeDriver,
    cfg: &CdcAcmConfig<'d>,
    vbus_detection: bool,
    bufs: &'d mut CdcAcmBuffersWithEpOut<'d>,
    state: &'d mut State<'d>,
) -> CdcAcm<'d, D>
where
    D: Driver<'d>,
    MakeDriver: FnOnce(&'d mut [u8], bool) -> D,
{
    let CdcAcmBuffersWithEpOut {
        ep_out,
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control,
    } = bufs;

    let driver = make_driver(ep_out, vbus_detection);
    let cdc_acm_bufs = CdcAcmBuffers {
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control,
    };
    build_cdc_acm(driver, cfg, cdc_acm_bufs, state)
}
