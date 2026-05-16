#![no_std]
#![no_main]

use bsp::hal::bind_interrupts;
use bsp::hal::dma;
use bsp::hal::exti;
use bsp::hal::gpio::Level;
use bsp::hal::gpio::Output;
use bsp::hal::gpio::Pull;
use bsp::hal::gpio::Speed;
use bsp::hal::interrupt;
use bsp::hal::peripherals;
use bsp::hal::spi;
use embassy_time::Duration;
use embassy_time::Timer;
use panic_halt as _;
use speedybee_f405_v4_bsp as bsp;

bind_interrupts!(struct Irqs {
    DMA2_STREAM3 => dma::InterruptHandler<peripherals::DMA2_CH3>;
    DMA2_STREAM0 => dma::InterruptHandler<peripherals::DMA2_CH0>;
    EXTI4 => exti::InterruptHandler<interrupt::typelevel::EXTI4>;
});

#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = bsp::hal::init(bsp::config());
    let board = bsp::Board::new(p);

    let mut led0 = Output::new(board.leds.led0, Level::Low, Speed::Low);
    let mut spi_cfg = spi::Config::default();
    spi_cfg.frequency = bsp::hal::time::mhz(10);
    let Some(spi1_rx) = board.dma.spi1.rx else {
        loop {
            led0.toggle();
            Timer::after(Duration::from_millis(250)).await;
        }
    };

    let mut spi = spi::Spi::new(
        board.imu_primary.spi,
        board.imu_primary.sck,
        board.imu_primary.mosi,
        board.imu_primary.miso,
        board.dma.spi1.tx,
        spi1_rx,
        Irqs,
        spi_cfg,
    );

    let mut cs = Output::new(board.imu_primary.cs, Level::High, Speed::Low);
    let mut drdy = exti::ExtiInput::new(
        board.imu_primary.int,
        board.imu_primary.int_exti,
        Pull::None,
        Irqs,
    );

    loop {
        drdy.wait_for_rising_edge().await;

        let mut whoami = [0x75u8 | 0x80u8, 0x00u8];
        cs.set_low();
        let read_ok = spi.transfer_in_place(&mut whoami).await.is_ok();
        cs.set_high();

        let valid = read_ok && (whoami[1] == 0x47 || whoami[1] == 0xDB);
        if valid {
            led0.set_low();
            Timer::after(Duration::from_millis(50)).await;
            led0.set_high();
            Timer::after(Duration::from_millis(50)).await;
        } else {
            led0.toggle();
            Timer::after(Duration::from_millis(200)).await;
        }
    }
}
