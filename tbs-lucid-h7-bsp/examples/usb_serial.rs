#![no_std]
#![no_main]

use cortex_m as _;
use defmt::info;
use defmt_rtt as _;
use embassy_futures::join::join;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use embassy_usb::class::cdc_acm::State;
use panic_probe as _;
use tbs_lucid_h7_bsp as bsp;

async fn serial_writer<'a>(class: &mut CdcAcmClass<'a, bsp::usb::FsDriver<'a>>) -> ! {
    let mut counter: u32 = 0;
    loop {
        class.wait_connection().await;
        info!("usb host connected");

        loop {
            counter = counter.wrapping_add(1);
            let digit = [b'0' + (counter % 10) as u8];

            let write_result = async {
                class.write_packet(b"tbs usb serial tick ").await?;
                class.write_packet(&digit).await?;
                class.write_packet(b"\r\n").await
            }
            .await;

            match write_result {
                Ok(()) => {}
                Err(_) => {
                    info!("usb host disconnected");
                    break;
                }
            }

            Timer::after(Duration::from_secs(1)).await;
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    info!("starting usb_serial example");

    let p = bsp::hal::init(bsp::config_with_usb());
    let board = bsp::Board::new(p);

    let mut ep_out = [0u8; 256];
    let mut config_descriptor = [0u8; 256];
    let mut bos_descriptor = [0u8; 256];
    let mut msos_descriptor = [0u8; 256];
    let mut control = [0u8; 64];

    let mut bufs = bsp::usb::UsbCdcAcmBuffers {
        ep_out: &mut ep_out,
        config_descriptor: &mut config_descriptor,
        bos_descriptor: &mut bos_descriptor,
        msos_descriptor: &mut msos_descriptor,
        control: &mut control,
    };

    let cfg = bsp::usb::UsbCdcAcmConfig::default();
    let mut state = State::new();

    let usb = board.usb.into_usb_cdc_acm(&cfg, &mut bufs, &mut state);
    let bsp::usb::UsbCdcAcm {
        mut class,
        mut device,
    } = usb;

    info!("usb CDC-ACM configured");
    join(device.run(), serial_writer(&mut class)).await;
}
