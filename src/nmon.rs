use crate::uart16550::Uart16550;

const HELP_MSG: &[u8] = b"\r\n\r\nnmon commands:\r\n q - quit\r\n d <addr> - read 32-bit word from <addr>\r\n w <addr> <val> - write 32-bit word to <addr>\r\n g <addr> - jump to <addr>\r\n   use <ESC> key to interrupt current command\r\n";
const PROMPT_MSG: &[u8] = b"\r\nnmon> ";

fn get_hex_digit(uart: &Uart16550) -> Option<u8> {
    loop {
        let c = uart.getc();
        if c == 0x1b { // ESCAPE
            return None;
        }
        if (c >= b'0') && (c <= b'9') {
            uart.outc(c);
            return Some(c - b'0');
        }
        if (c >= b'a') && (c <= b'f') {
            uart.outc(c);
            return Some(c - b'a' + 10);
        }
    }
}

fn get_hex_word(uart: &Uart16550) -> Option<u32> {
    let mut value: u32 = 0;
    for _ in 0..8 {
        value = value << 4;
        value |= get_hex_digit(uart)? as u32;
    }
    Some(value)
}

fn out_hex_word(uart: &Uart16550, value: u32) {
    for i in 0..8 {
        let b = ((value >> ((7 - i) * 4)) & 0xf)as u8;
        let c = if b < 10 {
            b + b'0'
        } else {
            b - 10 + b'a'
        };
        uart.outc(c);
    }
}

pub fn nmon(uart: &Uart16550) -> Option<()> {
    uart.outs(HELP_MSG);

    loop {
        uart.outs(PROMPT_MSG);
        let c = uart.getc();
        uart.outc(c);
        match c {
            b'q' => {
                uart.outnl();
                return None;
            }
            b'd' => {
                uart.outc(b' ');
                let address = get_hex_word(uart)?;
                uart.outnl();

                let value = unsafe { *(address as *const u32) };
                out_hex_word(uart, value);
            }
            b'w' => {
                uart.outc(b' ');
                let address = get_hex_word(uart)?;
                uart.outc(b' ');
                let value = get_hex_word(uart)?;

                unsafe { *(address as *mut u32) = value; }
            }
            b'g' => {
                uart.outc(b' ');
                let address = get_hex_word(uart)?;
                uart.outnl();

                let code: extern "C" fn() = unsafe { core::mem::transmute(address as usize) };
                (code)();
            }
            _ => return None
        }
    }
}
