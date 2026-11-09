//! Driver-free hardware verification. See the workspace docs/self-check.md.
#![no_std]
#![no_main]
use bsp::core::self_test::Report;
use bsp::core::self_test::blink_status_leds;
use bsp::core::self_test::serve_usb;
use panic_halt as _;
use tbs_lucid_h7_bsp as bsp;

bsp::bind_board_interrupts!(struct Irqs {
    usb_fs;
    primary_imu;
    secondary_imu;
    adc1;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let board = bsp::Board::new(bsp::hal::init(bsp::config_with_usb()));
    let (parts, usb) = bsp::self_test::split(board);
    let mut report = Report::<4096>::new();
    let mut leds = bsp::self_test::run(parts, Irqs, &mut report).await;

    let mut ep_out = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control = [0; 64];
    let mut buffers = bsp::usb::UsbCdcAcmBuffers {
        ep_out: &mut ep_out,
        config_descriptor: &mut config_descriptor,
        bos_descriptor: &mut bos_descriptor,
        msos_descriptor: &mut msos_descriptor,
        control: &mut control,
    };
    let mut state = embassy_usb::class::cdc_acm::State::new();
    let config = bsp::usb::UsbCdcAcmConfig {
        product: Some("TBS Lucid H7 self-check"),
        ..Default::default()
    };
    let cdc = usb.into_usb_cdc_acm(Irqs, &config, &mut buffers, &mut state);
    let status = report.status();
    let indicate = async {
        match blink_status_leds(&mut leds, &bsp::self_test::LED_POLARITIES, status).await {
            Ok(()) => {}
            Err(error) => match error {},
        }
    };
    embassy_futures::join::join(serve_usb(cdc, &report), indicate).await;
}
