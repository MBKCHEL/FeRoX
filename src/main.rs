#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
fn serial_write_byte(b: u8) {
    unsafe {
        asm!(
        "out dx, al",
        in("dx") 0x3F8u16,
        in("al") b,
        options(nomem, nostack, preserves_flags),
        );
    }
}

fn serial_write(s: &str) {
    for b in s.bytes() {
        serial_write_byte(b);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_write("Hello from FeRoX!\n");

    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_write("PANIC\n");
    loop {
        unsafe { asm!("hlt") };
    }
}