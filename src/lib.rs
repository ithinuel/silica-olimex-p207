#![no_std]

extern crate silica_stm32f207;

pub use silica_stm32f207::*;

// RS232_1_TX => C6
pub const RS232_1_TX: gpio::PinPeripheral<'static> = gpio::PinPeripheral {
    port: &GPIOPORTG,
    pin: 6,
    mode: gpio::Mode::AlternateFunction(gpio::AlternateFunction::AF8),
    speed: gpio::Frequency::F50MHz,
    pull_side: gpio::PullSide::Up
};
// RS232_1_RX => G9
pub const RS232_1_RX: gpio::PinPeripheral<'static> = gpio::PinPeripheral {
    port: &GPIOPORTG,
    pin: 9,
    mode: gpio::Mode::AlternateFunction(gpio::AlternateFunction::AF8),
    speed: gpio::Frequency::F50MHz,
    pull_side: gpio::PullSide::Up
};

// RS232_1 => USART6
pub static RS232_1: usart::USARTPeripheral<'static> = usart::USARTPeripheral {
    base_address: USART6,
    clock: rcc::RCCPeripheral { rcc: RCC, clock: rcc::Clock::USART6 },
    isr_id: IRQType::USART6,
    dma_rx: None,
    dma_tx: None,
    pin_tx: Some(&RS232_1_TX),
    pin_rx: Some(&RS232_1_RX),
    pin_dtr: None,
    pin_dcd: None,
    pin_dsr: None,
    pin_ri: None,
    pin_rts: None,
    pin_cts: None
};

pub fn init() {

}
