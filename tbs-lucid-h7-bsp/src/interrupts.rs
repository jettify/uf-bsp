//! Interrupt bindings are emitted only when the application invokes [`crate::bind_board_interrupts!`].
//!
//! Constructors also accept bindings from `hal::bind_interrupts!` directly.

/// Bind selected board peripherals in the application crate.
///
/// Supported selections: `primary_imu`, `secondary_imu`, `osd`, `receiver`, `usb_fs`.
/// Each selection ends with `;`. Only select groups whose default DMA mapping
/// you use. Invoke once per application (or use disjoint groups in separate invocations).
/// For custom vectors, omit the corresponding group and use `hal::bind_interrupts!`
/// in a separate invocation with its own binding type.
/// Each vector must have exactly one definition, including any shared handlers.
/// EXTI and executor interrupts remain application-owned and are not selected here.
///
/// ```ignore
/// bsp::bind_board_interrupts!(pub struct Irqs {
///     primary_imu;
///     usb_fs;
/// });
/// let imu = board.imu_primary.new_spi(Irqs, spi_config);
/// ```
#[macro_export]
macro_rules! bind_board_interrupts {
    ($(#[$attr:meta])* $vis:vis struct $name:ident { $($groups:tt)* }) => {
        $crate::bind_board_interrupts!(@collect [$(#[$attr])* $vis struct $name] [] $($groups)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*]) => {
        $crate::hal::bind_interrupts!($($decl)* { $($bindings)* });
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] primary_imu; $($rest:tt)*) => {
        $crate::bind_board_interrupts!(@collect [$($decl)*] [
            $($bindings)*
            DMA1_STREAM0 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH0>;
            DMA1_STREAM1 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH1>;
        ] $($rest)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] secondary_imu; $($rest:tt)*) => {
        $crate::bind_board_interrupts!(@collect [$($decl)*] [
            $($bindings)*
            DMA1_STREAM6 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH6>;
            DMA1_STREAM7 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH7>;
        ] $($rest)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] osd; $($rest:tt)*) => {
        $crate::bind_board_interrupts!(@collect [$($decl)*] [
            $($bindings)*
            DMA1_STREAM2 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH2>;
            DMA1_STREAM3 => $crate::hal::dma::InterruptHandler<$crate::hal::peripherals::DMA1_CH3>;
        ] $($rest)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] receiver; $($rest:tt)*) => {
        $crate::bind_board_interrupts!(@collect [$($decl)*] [
            $($bindings)*
            USART6 => $crate::hal::usart::InterruptHandler<$crate::hal::peripherals::USART6>;
        ] $($rest)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] usb_fs; $($rest:tt)*) => {
        $crate::bind_board_interrupts!(@collect [$($decl)*] [
            $($bindings)*
            OTG_FS => $crate::hal::usb::InterruptHandler<$crate::hal::peripherals::USB_OTG_FS>;
        ] $($rest)*);
    };
    (@collect [$($decl:tt)*] [$($bindings:tt)*] $unknown:ident; $($rest:tt)*) => {
        compile_error!(concat!("unknown board interrupt group: ", stringify!($unknown)));
    };
}
