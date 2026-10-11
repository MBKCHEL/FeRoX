#![no_std]
#![no_main]

use core::arch::asm;
use core::panic::PanicInfo;
use core::fmt::{Write};
use limine::{BaseRevision, request::FramebufferRequest};

#[used]
#[unsafe(no_mangle)]
static BASE_REVISION: BaseRevision = BaseRevision::new();
struct Serial;
#[used]
static FMB_REQUEST: FramebufferRequest = FramebufferRequest::new();

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
    let _ = Serial.write_fmt(args);
}

fn print_pixels(){
    if let Some(response) = FMB_REQUEST.response() {
        if let Some(fb) = response.framebuffers().first() {
            let start_ptr = fb.address() as *mut u32;

            let x = 50;
            let y = 50;

            let pixel_pos: u32 = (y * (fb.pitch / 4) + x).try_into().unwrap();
            sprintln!("Width: {}, Height: {}, Pitch: {}, bpp: {}", fb.width, fb.height, fb.pitch, fb.bpp);


            let white_color: u32 = 0x00FFFFFF;
            unsafe {
                let start_ptr = start_ptr.add(pixel_pos.try_into().unwrap());
                core::ptr::write_volatile(start_ptr, white_color);
            }
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if !BASE_REVISION.is_supported() {
        sprintln!("Unsupported revision");
    }
    else {
        sprintln!("Revision is Ok");
        print_pixels();
    }
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