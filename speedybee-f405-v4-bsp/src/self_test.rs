//! Driver-free board diagnostics, enabled by `self-test`.
//!
//! Interrupt handlers and HAL initialization remain application-owned. Individual
//! checks consume only their resource group. `split` is a convenience for standalone
//! firmware: unused board resources are discarded. No sensor is initialized.
use crate::core::capabilities::PinPolarity;
use crate::core::self_test as checks;
use crate::core::self_test::CheckOutcome;
use crate::core::self_test::ChipId;
use crate::core::self_test::I2cChipId;
use crate::core::self_test::Report;
use crate::hal;
use crate::parts;
use embassy_time::Duration;
use embedded_hal_bus::spi::ExclusiveDevice;
use hal::gpio::Level;
use hal::gpio::Output;
use hal::gpio::Speed;

pub const LED_POLARITIES: [Option<PinPolarity>; 1] = [parts::Leds::LED0_POLARITY];

/// Application binding proof for the board's default ADC1 DMA channel.
pub trait Adc1Irqs:
    hal::interrupt::typelevel::Binding<
        hal::interrupt::typelevel::DMA2_STREAM0,
        hal::dma::InterruptHandler<hal::peripherals::DMA2_CH0>,
    >
{
}
impl<T> Adc1Irqs for T where
    T: hal::interrupt::typelevel::Binding<
            hal::interrupt::typelevel::DMA2_STREAM0,
            hal::dma::InterruptHandler<hal::peripherals::DMA2_CH0>,
        >
{
}

pub const IMU_CHIP_IDS: &[ChipId] = &[ChipId {
    value: 0x47,
    mask: 0xff,
    name: "ICM42688",
}];

pub const BARO_CHIP_IDS: &[I2cChipId] = &[
    I2cChipId {
        address: 0x77,
        register: 0x0d,
        chip: ChipId {
            value: 0x10,
            mask: 0xf0,
            name: "DPS310",
        },
    },
    I2cChipId {
        address: 0x76,
        register: 0x0d,
        chip: ChipId {
            value: 0x10,
            mask: 0xf0,
            name: "DPS310",
        },
    },
];

pub struct Parts<'d> {
    pub leds: parts::Leds<'d>,
    pub imu_primary: parts::PrimaryImu<'d>,
    pub baro: parts::BaroParts<'d>,
    pub adc: parts::AdcParts<'d>,
    pub adc_dma: crate::dma::Adc1Dma<'d>,
}

pub fn split(board: crate::Board<'_>) -> (Parts<'_>, parts::UsbParts<'_>) {
    (
        Parts {
            leds: board.leds,
            imu_primary: board.imu_primary,
            baro: board.baro,
            adc: board.adc,
            adc_dma: board.dma.adc1,
        },
        board.usb,
    )
}

pub async fn check_primary_imu<'d, I: parts::PrimaryImuIrqs + 'd, const N: usize>(
    parts: parts::PrimaryImu<'d>,
    irqs: I,
    report: &mut Report<N>,
) -> CheckOutcome {
    let mut config = hal::spi::Config::default();
    config.frequency = hal::time::Hertz::mhz(1);
    let imu = parts.new_spi(irqs, config);
    let cs = Output::new(imu.cs, Level::High, Speed::Low);
    let mut device = match ExclusiveDevice::new_no_delay(imu.spi, cs) {
        Ok(device) => device,
        Err(_) => return CheckOutcome::fail("SPI device construction failed"),
    };
    // WHO_AM_I 0x75, with the device's SPI read bit. No EXTI is enabled.
    checks::probe_spi_chip_id(&mut device, 0xf5, IMU_CHIP_IDS, false, report).await
}

pub fn check_barometer<const N: usize>(
    parts: parts::BaroParts<'_>,
    report: &mut Report<N>,
) -> CheckOutcome {
    let mut config = hal::i2c::Config::default();
    config.frequency = hal::time::Hertz::khz(400);
    config.timeout = Duration::from_millis(50);
    let mut bus = parts.new_i2c_blocking(config);
    checks::probe_i2c_chip_id(&mut bus, BARO_CHIP_IDS, false, report)
}

pub async fn check_adc<'d, I: Adc1Irqs + 'd, const N: usize>(
    parts: parts::AdcParts<'d>,
    dma: crate::dma::Adc1Dma<'d>,
    irqs: I,
    report: &mut Report<N>,
) -> CheckOutcome {
    use hal::adc::AdcChannel as _;
    const CHANNELS: usize = 3;
    const DMA_SAMPLES: usize = 192;
    // Embassy ring-buffered reads must consume exactly half the DMA ring.
    const READ_SAMPLES: usize = DMA_SAMPLES / 2;
    const _: () = assert!(
        READ_SAMPLES.is_multiple_of(CHANNELS),
        "whole ADC scans required"
    );
    let mut buffer = [0; DMA_SAMPLES];
    let mut adc = hal::adc::Adc::new(parts.adc1).into_ring_buffered(
        dma.ch,
        &mut buffer,
        irqs,
        [
            parts.vbat.degrade_adc(),
            parts.current.degrade_adc(),
            parts.rssi.degrade_adc(),
        ]
        .into_iter()
        .map(|channel| (channel, hal::adc::SampleTime::CYCLES480)),
        hal::adc::CONTINUOUS,
        hal::adc::Exten::DISABLED,
    );
    let mut samples = [0; READ_SAMPLES];
    let mut reads = 0;
    let until = embassy_time::Instant::now() + Duration::from_millis(200);
    while embassy_time::Instant::now() < until {
        if adc.read(&mut samples).await.is_err() {
            return CheckOutcome::fail("ADC DMA ring read/overrun failed");
        }
        reads += 1;
    }
    report.note(format_args!(
        "ADC completed_reads={} raw [vbat,current,rssi]={:?}",
        reads,
        samples.first_chunk::<CHANNELS>()
    ));
    checks::validate_adc_samples(reads, &samples, 12)
}

/// Run the board's baseline suite sequentially. Returns the configured LEDs for
/// the application's status display.
pub async fn run<'d, I, const N: usize>(
    parts: Parts<'d>,
    irqs: I,
    report: &mut Report<N>,
) -> [Output<'d>; 1]
where
    I: parts::PrimaryImuIrqs + Adc1Irqs + Copy + 'd,
{
    report.note(format_args!(
        "SpeedyBee F405 V4 self-check v2: identities and peripherals; no sensor drivers"
    ));
    let mut leds = [Output::new(parts.leds.led0, Level::Low, Speed::Low)];
    report
        .run_check("LEDs", async |_| {
            checks::check_leds(&mut leds, &LED_POLARITIES).await
        })
        .await;
    report
        .run_check("timer", async |_| checks::check_timer().await)
        .await;
    report
        .run_check("primary IMU identity", async |r| {
            check_primary_imu(parts.imu_primary, irqs, r).await
        })
        .await;
    report.run_sync_check("barometer identity", |r| check_barometer(parts.baro, r));
    report
        .run_check("ADC", async |r| {
            check_adc(parts.adc, parts.adc_dma, irqs, r).await
        })
        .await;
    report.record(
        "sensor sampling/DRDY",
        CheckOutcome::skip("identity-only checks; use sensor examples"),
    );
    report.record(
        "SD/OSD",
        CheckOutcome::skip("outside the driver-free baseline"),
    );
    leds
}
