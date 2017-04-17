#![no_std]

extern crate silica_stm32f2xx;
extern crate silica_stm32f207;

pub use silica_stm32f207::*;
pub use silica_stm32f207::gpio::*;


// RS232_1_TX => C6
pub const RS232_1_TX: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTG,
    pin: 6,
    mode: Mode::AlternateFunction(AlternateFunction::AF8),
    speed: Frequency::F50MHz,
    pull_side: PullSide::Up
};
// RS232_1_RX => G9
pub const RS232_1_RX: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTG,
    pin: 9,
    mode: Mode::AlternateFunction(AlternateFunction::AF8),
    speed: Frequency::F50MHz,
    pull_side: PullSide::Up
};

pub static STAT1_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTF,
    pin: 6,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT2_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTF,
    pin: 7,
    mode: Mode::Out(OutputType::PushPull, false),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT3_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTF,
    pin: 8,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT4_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIOPORTF,
    pin: 9,
    mode: Mode::Out(OutputType::PushPull, false),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
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

pub fn init() -> Result<u32, &'static str> {
    system_init(
        rcc::ClockSelection::HSE(25_000_000),
        rcc::PLL::On(25, 240, 2, 5),
        rcc::CFGR_HPrescaler::Div1,
        rcc::CFGR_PPrescaler1::Div4,
        rcc::CFGR_PPrescaler2::Div2
    )
}
