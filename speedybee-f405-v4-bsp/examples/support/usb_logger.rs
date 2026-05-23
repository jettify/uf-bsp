use embassy_futures::join::join;
use embassy_usb::class::cdc_acm::State;
use static_cell::StaticCell;

use speedybee_f405_v4_bsp as bsp;

const LOGGER_BUFFER_SIZE: usize = 1024;
const LOGGER_LEVEL: log::LevelFilter = log::LevelFilter::Info;
const DEFAULT_STARTUP_WINDOW_MS: u64 = 1_000;

pub fn init(
    usb: bsp::parts::UsbParts<'static>,
    product: &'static str,
) -> bsp::usb::UsbCdcAcm<'static> {
    let mut cfg = bsp::usb::UsbCdcAcmConfig::default();
    cfg.product = Some(product);
    cfg.max_packet_size = embassy_usb_logger::MAX_PACKET_SIZE as u16;

    usb.into_cdc_acm(&cfg, new_buffers(), new_state())
}

pub struct UsbLog {
    startup_window_ms: u64,
}

impl UsbLog {
    pub async fn wait_startup(&self) {
        embassy_time::Timer::after(embassy_time::Duration::from_millis(self.startup_window_ms))
            .await;
    }
}

#[embassy_executor::task]
async fn usb_log_task(usb_cdc: bsp::usb::UsbCdcAcm<'static>) -> ! {
    let bsp::usb::UsbCdcAcm {
        mut device,
        class: logger_class,
    } = usb_cdc;
    let log_fut = embassy_usb_logger::with_class!(LOGGER_BUFFER_SIZE, LOGGER_LEVEL, logger_class);

    join(device.run(), log_fut).await
}

pub fn spawn(
    spawner: &embassy_executor::Spawner,
    usb: bsp::parts::UsbParts<'static>,
    product: &'static str,
    startup_window_ms: u64,
) -> UsbLog {
    let usb_cdc = init(usb, product);
    spawner.must_spawn(usb_log_task(usb_cdc));
    UsbLog { startup_window_ms }
}

pub fn spawn_default(
    spawner: &embassy_executor::Spawner,
    usb: bsp::parts::UsbParts<'static>,
    product: &'static str,
) -> UsbLog {
    spawn(spawner, usb, product, DEFAULT_STARTUP_WINDOW_MS)
}

fn new_state() -> &'static mut State<'static> {
    static STATE: StaticCell<State<'static>> = StaticCell::new();
    STATE.init(State::new())
}

fn new_buffers() -> &'static mut bsp::usb::UsbCdcAcmBuffers<'static> {
    static EP_OUT: StaticCell<[u8; 256]> = StaticCell::new();
    static CONFIG_DESCRIPTOR: StaticCell<[u8; 128]> = StaticCell::new();
    static BOS_DESCRIPTOR: StaticCell<[u8; 16]> = StaticCell::new();
    static MSOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
    static CONTROL: StaticCell<[u8; 64]> = StaticCell::new();
    static BUFFERS: StaticCell<bsp::usb::UsbCdcAcmBuffers<'static>> = StaticCell::new();

    BUFFERS.init(bsp::usb::UsbCdcAcmBuffers {
        ep_out: EP_OUT.init([0; 256]),
        config_descriptor: CONFIG_DESCRIPTOR.init([0; 128]),
        bos_descriptor: BOS_DESCRIPTOR.init([0; 16]),
        msos_descriptor: MSOS_DESCRIPTOR.init([0; 256]),
        control: CONTROL.init([0; 64]),
    })
}
