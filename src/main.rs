#![no_std]
#![no_main]

mod console;
mod graphics;
mod keyboard;
mod scancode;

use crate::keyboard::read_keyboard_scancode;
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use limine::BaseRevision;

#[used]
#[unsafe(no_mangle)]
static BASE_REVISION: BaseRevision = BaseRevision::new();
struct Serial;

impl Write for Serial {
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
    ($($arg:tt)*) => { $crate::_print(format_args!("{}\n", format_args!($($arg)*))) };
}

fn _print(args: core::fmt::Arguments) {
    let _ = Serial.write_fmt(args);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if !BASE_REVISION.is_supported() {
        sprint!("Unsupported revision");
    } else {
        graphics::buffer_info();
        unsafe {
            if let Some(ref fb) = graphics::FB_GLOBAL {
                sprintln!(
                    "Width: {}, Height: {}, Pitch: {}, bpp: {}",
                    fb.width,
                    fb.height,
                    fb.pitch,
                    fb.bpp
                );
            }
        }

        kprint!(
            "Hello from FeRoX Kernel! | MBKCHEL age: {} 12345678901234567890123456789012345678901234567890",
            109
        );

        sprintln!("Revision is Ok");
    }
    loop {
        if let Some(symbol) = read_keyboard_scancode() {
            kprint!("{}", symbol);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    kprint!("{}", _info);
    loop {
        unsafe { asm!("hlt") };
    }
}
