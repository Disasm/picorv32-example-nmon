use volatile_register::{RO, WO, RW};

#[repr(C)]
#[allow(non_snake_case)]
pub struct Uart16550 {
    DATA: RW<u32>,
    IER: RW<u32>,
    IIR_FCR: RW<u32>,
    LCR: RW<u32>,
    MCR: WO<u32>,
    LSR: RO<u32>,
    MSR: RO<u32>,
}

impl Uart16550 {
    pub fn new(address: u32) -> &'static Self {
        unsafe { & *(address as *const Self) }
    }

    pub fn init(&self, divisor: u16) {
        unsafe {
            self.LCR.write(0x87); // DLAB on
            self.DATA.write((divisor & 0xff) as u32); // write low order byte
            self.IER.write((divisor >> 8) as u32); // write high order byte
            self.LCR.write(0x07); // DLAB off
        }
    }

    pub fn outc(&self, c: u8) {
        unsafe {
            while (self.LSR.read() & 0x20) == 0 {}  // check for transmitter empty
            self.DATA.write(c as u32);
        }
    }

    pub fn outs(&self, s: &[u8]) {
        for c in s {
            self.outc(*c);
        }
    }

    pub fn outnl(&self) {
        self.outc(b'\r');
        self.outc(b'\n');
    }

    pub fn tstc(&self) -> bool {
        self.LSR.read() & 0x01 != 0
    }

    pub fn getc(&self) -> u8 {
        while !self.tstc() {}
        (self.DATA.read() & 0xff) as u8
    }
}
