use crate::scancode::scan_code_check;
use x86_64::instructions::port::Port;

pub fn read_keyboard_scancode() -> Option<char> {
    let mut data_port = Port::<u8>::new(0x60);
    let mut status_port = Port::<u8>::new(0x64);

    let status = unsafe { status_port.read() };

    if (status & 0x1) != 0 {
        let scancode = unsafe { data_port.read() };
        if scancode & 0x80 != 0 {
            return None;
        }

        let symbol = scan_code_check(scancode);
        Some(symbol)
    } else {
        None
    }
}
