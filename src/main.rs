#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::fmt::{Write};

struct Serial;

impl Write for Serial{
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            unsafe {
                asm!(
                "out dx, al",
                in("dx") 0x3F8u16,
                in("al") b,
                options(nomem, nostack, preserves_flags),
                );
            }
        }
        Ok(())
    }
}

macro_rules! sprint {
    ($($arg:tt)*) => { _print(format_args!($($arg)*)) };
}

macro_rules! sprintln {
    ($($arg:tt)*) => { _print(format_args!("{}\n", format_args!($($arg)*))) };
}

fn _print(args: core::fmt::Arguments) {
    Serial.write_fmt(args).unwrap();
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let my_number = 3232;

    sprintln!("{}", my_number);
    sprint!("test");
    loop {
        unsafe { asm!("hlt") };
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    sprint!("{}", _info);
    loop {
        unsafe { asm!("hlt") };
    }
}