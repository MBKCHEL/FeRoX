#![no_std]
#![no_main]

use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use limine::{BaseRevision, request::FramebufferRequest};

#[used]
#[unsafe(no_mangle)]
static BASE_REVISION: BaseRevision = BaseRevision::new();
struct Serial;
#[used]
static FMB_REQUEST: FramebufferRequest = FramebufferRequest::new();

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
    ($($arg:tt)*) => { _print(format_args!("{}\n", format_args!($($arg)*))) };
}

fn _print(args: core::fmt::Arguments) {
    let _ = Serial.write_fmt(args);
}

fn fill_rect(x: u64, y: u64, width: u64, height: u64, color: u32) {
    for dy in 0..height {
        for dx in 0..width {
            put_pixel(x + dx, y + dy, color);
        }
    }
}

fn print_letter(x: u64, y: u64, color: u32, scale: u64) {
    let matrix: [[u8; 1]; 8] = [
        [0b00011000], [0b00111100], [0b01100110], [0b11000011], [0b11111111], [0b11000011], [0b11000011], [0b00000000]
    ];
    let height = scale;
    for dy in 0..8 {
        for dx in 0..8 {
            if (matrix[dy][0] >> 7 -dx) & 1 == 1{
                fill_rect( x + dx as u64 * scale, y + dy as u64 * scale, scale, height, color);
            }
        }
    }
}

pub struct Framebuffer {
    pub address: *mut u32,
    pub height: u64,
    pub width: u64,
    pub pitch: u64,
    pub bpp: u64,
}

static mut FB_GLOBAL: Option<Framebuffer> = None;

fn buffer_info() {
    if let Some(response) = FMB_REQUEST.response() {
        if let Some(fb) = response.framebuffers().first() {
            let fb_instance = Framebuffer {
                address: fb.address() as *mut u32,
                height: fb.height,
                width: fb.width,
                pitch: fb.pitch,
                bpp: fb.bpp as u64,
            };

            unsafe {
                FB_GLOBAL = Some(fb_instance);
            }
        }
    }
}

fn put_pixel(x: u64, y: u64, color: u32) {
    unsafe {
        if let Some(ref fb) = FB_GLOBAL {
            let start_ptr = fb.address;

            let pixel_pos = y * (fb.pitch / 4) + x;

            if y >= fb.height {
                return;
            }
            if x >= fb.width {
                return;
            }

            let start_ptr = start_ptr.add(pixel_pos as usize);
            core::ptr::write_volatile(start_ptr, color);
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    if !BASE_REVISION.is_supported() {
        sprintln!("Unsupported revision");
    } else {
        buffer_info();
        unsafe {
            if let Some(ref fb) = FB_GLOBAL {
                sprintln!(
                    "Width: {}, Height: {}, Pitch: {}, bpp: {}",
                    fb.width,
                    fb.height,
                    fb.pitch,
                    fb.bpp
                );
            }
        }

        sprintln!("Revision is Ok");

        let x = 50;
        let y = 600;
        let color = 0xFF0000;
        let scale = 2;
        print_letter(x, y, color, scale);
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