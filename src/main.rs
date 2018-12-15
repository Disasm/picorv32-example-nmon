#![no_main]
#![no_std]

extern crate panic_halt;
use picorv32_rt::entry;

entry!(main);
fn main() -> ! {
    loop {}
}
