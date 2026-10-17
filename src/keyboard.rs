use crate::scancode::scan_code_check;
use x86_64::instructions::port::Port;
use x86_64::instructions::port::{PortGeneric, ReadWriteAccess};

pub fn read_keyboard_scancode() -> Option<char> {
    let mut data_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);
    let mut status_port: PortGeneric<u8, ReadWriteAccess> = Port::new(0x64);

    let status: u8 = unsafe { status_port.read() };

    if (status & 0x1) != 0 {
        let scancode: u8 = unsafe { data_port.read() };

        if scancode & 0x80 != 0 {
            return None;
        }

        scan_code_check(scancode)
    } else {
        None
    }
}
