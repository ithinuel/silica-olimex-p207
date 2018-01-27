#![no_std]
#![feature(drop_types_in_const)]

extern crate silica_stm32f2xx;
extern crate silica_stm32f207;

pub use silica_stm32f207::*;
use silica_stm32f207::gpio::*;

// RS232_1_TX => C6
pub static RS232_1_TX: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_C,
    pin: 6,
    mode: Mode::AlternateFunction(AlternateFunction::AF8, OutputType::PushPull),
    speed: Frequency::F50MHz,
    pull_side: PullSide::Up
};
// RS232_1_RX => G9
pub static RS232_1_RX: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_G,
    pin: 9,
    mode: Mode::AlternateFunction(AlternateFunction::AF8, OutputType::PushPull),
    speed: Frequency::F50MHz,
    pull_side: PullSide::Up
};

pub static STAT1_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_F,
    pin: 6,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT2_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_F,
    pin: 7,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT3_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_F,
    pin: 8,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static STAT4_E: PinPeripheral<'static> = PinPeripheral {
    port: &GPIO_F,
    pin: 9,
    mode: Mode::Out(OutputType::PushPull, true),
    speed: Frequency::F2MHz,
    pull_side: PullSide::Both
};

pub static RS232_1_DMA_RX: dma::DMAStreamPeripheral<'static> = dma::DMAStreamPeripheral {
    dma: &DMA_2,
    isr_id: irq::IRQType::DMA2_Stream1,
    stream: dma::Stream::Stream1,
    channel: dma::Channel::Channel5,
    priority: dma::Priority::Low
};
pub static RS232_1_DMA_TX: dma::DMAStreamPeripheral<'static> = dma::DMAStreamPeripheral {
    dma: &DMA_2,
    isr_id: irq::IRQType::DMA2_Stream6,
    stream: dma::Stream::Stream6,
    channel: dma::Channel::Channel5,
    priority: dma::Priority::Low
};

// RS232_1 => USART6
pub static RS232_1: usart::USARTPeripheral<'static> = usart::USARTPeripheral {
    base_address: USART_6_REGISTERS,
    clock: rcc::RCCPeripheral { rcc: RCC_REGISTERS, clock: rcc::Clock::USART6 },
    isr_id: irq::IRQType::USART6,
    dma_rx: Some(&RS232_1_DMA_RX),
    dma_tx: Some(&RS232_1_DMA_TX),
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
    rcc::system_init(
        rcc::ClockSelection::HSE(25_000_000),
        rcc::PLL::On(25, 240, 2, 5),
        rcc::CFGR_HPrescaler::Div1,
        rcc::CFGR_PPrescaler1::Div4,
        rcc::CFGR_PPrescaler2::Div2
    )
}
