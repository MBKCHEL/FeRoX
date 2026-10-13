use limine::request::FramebufferRequest;
use crate::console;
use core::fmt::Write;

#[used]
static FMB_REQUEST: FramebufferRequest = FramebufferRequest::new();


pub fn _kprint(args: core::fmt::Arguments) {
    let _ = console::CONSOLE.lock().write_fmt(args);
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => { $crate::graphics::_kprint(format_args!("{}\n", format_args!($($arg)*))) };
}

pub struct Framebuffer {
    pub address: *mut u32,
    pub height: u64,
    pub width: u64,
    pub pitch: u64,
    pub bpp: u64,
}

pub fn fill_rect(x: u64, y: u64, width: u64, height: u64, color: u32) {
    for dy in 0..height {
        for dx in 0..width {
            put_pixel(x + dx, y + dy, color);
        }
    }
}

pub fn buffer_info() {
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
                console::FB_GLOBAL = Some(fb_instance);
            }
        }
    }
}

pub fn put_pixel(x: u64, y: u64, color: u32) {
    unsafe {
        if let Some(ref fb) = console::FB_GLOBAL {
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

pub fn print_letters(x: u64, y: u64, color: u32, scale: u64, glyph: [u8; 8]) {
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