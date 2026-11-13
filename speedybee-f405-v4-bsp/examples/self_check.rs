//! Driver-free hardware verification. See docs/self-check.md.
#![no_std]
#![no_main]

use core::fmt::Write as _;

use bsp::hal;
use bsp::hal::adc::AdcChannel as _;
use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Speed;
use embassy_futures::join::join3;
use embassy_time::Duration;
use embassy_time::Timer;
use embassy_time::with_timeout;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

bsp::bind_board_interrupts!(struct Irqs {
    usb_fs;
    primary_imu;
    adc1;
});

type Report = heapless::String<512>;

fn log_line(out: &mut Report, args: core::fmt::Arguments<'_>) {
    let _ok = out.write_fmt(args).is_ok() && out.write_str("\r\n").is_ok();
}

async fn check_imu(parts: bsp::parts::PrimaryImu<'_>, out: &mut Report) -> bool {
    let mut config = hal::spi::Config::default();
    config.frequency = hal::time::Hertz::mhz(1);
    let mut imu = parts.new_spi(Irqs, config);
    let mut cs = Output::new(imu.cs, Level::High, Speed::Low);

    let mut buf = [0xf5u8, 0xff];
    cs.set_low();
    let res = with_timeout(
        Duration::from_millis(50),
        imu.spi.transfer_in_place(&mut buf),
    )
    .await;
    cs.set_high();
    let [_, id] = buf;

    match res {
        Ok(Ok(())) if id == 0x47 => {
            log_line(out, format_args!("PASS  imu_primary     ICM42688 (0x47)"));
            true
        }
        Ok(Ok(())) => {
            log_line(
                out,
                format_args!("FAIL  imu_primary     unexpected id 0x{id:02x}"),
            );
            false
        }
        _ => {
            log_line(
                out,
                format_args!("FAIL  imu_primary     SPI transfer failed"),
            );
            false
        }
    }
}

fn check_baro(parts: bsp::parts::BaroParts<'_>, out: &mut Report) -> bool {
    let mut config = hal::i2c::Config::default();
    config.frequency = hal::time::Hertz::khz(400);
    config.timeout = Duration::from_millis(50);
    let mut i2c = parts.new_i2c_blocking(config);

    for addr in [0x77, 0x76] {
        let mut buf = [0u8];
        if i2c.blocking_write_read(addr, &[0x0d], &mut buf).is_ok() {
            let [id] = buf;
            if (id & 0xf0) == 0x10 {
                log_line(
                    out,
                    format_args!("PASS  baro            DPS310 (addr=0x{addr:02x}, id=0x{id:02x})"),
                );
                return true;
            }
        }
    }
    log_line(
        out,
        format_args!("FAIL  baro            no DPS310 on 0x77/0x76"),
    );
    false
}

async fn check_adc(
    parts: bsp::parts::AdcParts<'_>,
    dma: bsp::dma::Adc1Dma<'_>,
    out: &mut Report,
) -> bool {
    const CHANNELS: usize = 3;
    const DMA_SAMPLES: usize = 192;
    const READ_SAMPLES: usize = DMA_SAMPLES / 2;
    const _: () = assert!(READ_SAMPLES.is_multiple_of(CHANNELS), "whole ADC scans");

    let mut buffer = [0u16; DMA_SAMPLES];
    let mut adc = hal::adc::Adc::new(parts.adc1).into_ring_buffered(
        dma.ch,
        &mut buffer,
        Irqs,
        [
            parts.vbat.degrade_adc(),
            parts.current.degrade_adc(),
            parts.rssi.degrade_adc(),
        ]
        .into_iter()
        .map(|ch| (ch, hal::adc::SampleTime::CYCLES480)),
        hal::adc::CONTINUOUS,
        hal::adc::Exten::DISABLED,
    );

    let mut samples = [0u16; READ_SAMPLES];
    if matches!(
        with_timeout(Duration::from_millis(250), adc.read(&mut samples)).await,
        Ok(Ok(_))
    ) {
        let [vbat, current, rssi, ..] = samples;
        log_line(
            out,
            format_args!("PASS  adc             vbat={vbat} current={current} rssi={rssi}"),
        );
        true
    } else {
        log_line(out, format_args!("FAIL  adc             DMA read failed"));
        false
    }
}

async fn send_report<'d>(
    class: &mut embassy_usb::class::cdc_acm::CdcAcmClass<'d, bsp::usb::FsDriver<'d>>,
    bytes: &[u8],
) -> bool {
    let packet_size = usize::from(class.max_packet_size()).max(1);
    for chunk in bytes.chunks(packet_size) {
        if !matches!(
            with_timeout(Duration::from_millis(250), class.write_packet(chunk)).await,
            Ok(Ok(()))
        ) {
            return false;
        }
    }
    if bytes.len().is_multiple_of(packet_size) {
        return matches!(
            with_timeout(Duration::from_millis(250), class.write_packet(&[])).await,
            Ok(Ok(()))
        );
    }
    true
}

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let board = bsp::Board::new(bsp::hal::init(bsp::config_with_usb()));
    let mut led = Output::new(board.leds.led0, Level::Low, Speed::Low);

    // Allow onboard sensors to finish power-on startup before probing registers.
    Timer::after_millis(100).await;

    let mut report = Report::new();
    log_line(
        &mut report,
        format_args!("\r\n--- {} ---", bsp::Board::NAME),
    );
    let imu_ok = check_imu(board.imu_primary, &mut report).await;
    let baro_ok = check_baro(board.baro, &mut report);
    let adc_ok = check_adc(board.adc, board.dma.adc1, &mut report).await;
    let passed = imu_ok && baro_ok && adc_ok;
    log_line(
        &mut report,
        format_args!("RESULT: {}", if passed { "PASS" } else { "FAIL" }),
    );

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
        product: Some(bsp::Board::NAME),
        ..Default::default()
    };
    let bsp::usb::UsbCdcAcm {
        mut device,
        mut class,
    } = board
        .usb
        .into_usb_cdc_acm(Irqs, &config, &mut buffers, &mut state);

    let usb_writer = async {
        loop {
            class.wait_connection().await;
            let sent = send_report(&mut class, report.as_bytes()).await;
            Timer::after_millis(if sent { 2000 } else { 500 }).await;
        }
    };

    let blink = async {
        let pulses = if passed { 1 } else { 3 };
        loop {
            for _ in 0..pulses {
                led.set_high();
                Timer::after_millis(150).await;
                led.set_low();
                Timer::after_millis(150).await;
            }
            Timer::after_secs(1).await;
        }
    };

    join3(device.run(), usb_writer, blink).await;
}
