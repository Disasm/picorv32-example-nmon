#![no_main]
#![no_std]

extern crate panic_halt;
use picorv32_rt::entry;

mod uart16550;
use crate::uart16550::Uart16550;

mod nmon;
use crate::nmon::nmon;


const SYS_CLK_FREQ: u32 = 10_000_000;
const UART_CLK: u32 = SYS_CLK_FREQ / 16;
const UART_BAUD_RATE: u32 = 9600;
const UART_DIVISOR: u16 = (UART_CLK / UART_BAUD_RATE) as u16;


entry!(main);
fn main() -> ! {
    let uart0 = Uart16550::new(0x9000_0000);
    uart0.init(UART_DIVISOR);

    loop {
        nmon(&uart0);
    }
}
