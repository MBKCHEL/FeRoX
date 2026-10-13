#![no_std]
#![no_main]
use core::arch::asm;
use core::fmt::Write;
use core::panic::PanicInfo;
use font8x8::{BASIC_FONTS, UnicodeFonts};
use limine::{BaseRevision, request::FramebufferRequest};
use spin::Mutex;

#[used]
#[unsafe(no_mangle)]
static BASE_REVISION: BaseRevision = BaseRevision::new();
struct Serial;
#[used]
static FMB_REQUEST: FramebufferRequest = FramebufferRequest::new();
#[used]
static CONSOLE: Mutex<Console> = Mutex::new(Console {
    x: 0,
    y: 0,
    color: 0xFFFFFF,
    scale: 2,
});

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

fn _kprint(args: core::fmt::Arguments) {
    let _ = CONSOLE.lock().write_fmt(args);
}

macro_rules! kprint {
    ($($arg:tt)*) => { _kprint(format_args!("{}\n", format_args!($($arg)*))) };
}

fn fill_rect(x: u64, y: u64, width: u64, height: u64, color: u32) {
    for dy in 0..height {
        for dx in 0..width {
            put_pixel(x + dx, y + dy, color);
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

struct Console {
    x: u64,
    y: u64,
    color: u32,
    scale: u64,
}
impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe {
                if let Some(ref fb) = FB_GLOBAL {
                    if c == '\n' {
                        self.x = 0;
                        self.y += 10 * self.scale;
                    } else {
                        if self.x + 8 * self.scale > fb.width {
                            self.x = 0;
                            self.y += 10 * self.scale;
                        }

                        if let Some(glyph_data) = BASIC_FONTS.get(c) {
                            let glyph: [u8; 8] = glyph_data;
                            print_letters(self.x, self.y, self.color, self.scale, glyph);
                        }

                        self.x += 8 * self.scale;
                    }
                }
            }
        }
        Ok(())
    }
}

fn print_letters(x: u64, y: u64, color: u32, scale: u64, glyph: [u8; 8]) {
    let height = scale;
    for dy in 0..8 {
        for dx in 0..8 {
            if (glyph[dy] >> dx) & 1 == 1 {
                fill_rect(
                    x + dx as u64 * scale,
                    y + dy as u64 * scale,
                    scale,
                    height,
                    color,
                );
            }
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

        kprint!( "Hello from FeRoX Kernel! | MBKCHEL age: {} 12345678901234567890123456789012345678901234567890", 109);

        sprintln!("Revision is Ok");
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
