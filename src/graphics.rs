use limine::request::FramebufferRequest;
use crate::console::{CONSOLE};
use core::fmt::Write;

pub static mut FB_GLOBAL: Option<Framebuffer> = None;
#[used]
static FMB_REQUEST: FramebufferRequest = FramebufferRequest::new();


pub fn _kprint(args: core::fmt::Arguments) {
    let _ = CONSOLE.lock().write_fmt(args);
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => { $crate::graphics::_kprint(format_args!("{}", format_args!($($arg)*))) };
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
                FB_GLOBAL = Some(fb_instance);
            }
        }
    }
}

pub fn put_pixel(x: u64, y: u64, color: u32) {
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
pub(crate) fn scroll_down(amount_pixels: u64) {
    unsafe {
        if let Some(ref fb) = FB_GLOBAL {
            if amount_pixels >= fb.height {
                fill_rect(0, 0, fb.width, fb.height, 0x000000);
                return;
            }

            let bytes_per_line = fb.pitch;

            let byte_offset = amount_pixels * bytes_per_line;

            let total_bytes = fb.height * bytes_per_line;

            let bytes_to_copy = total_bytes - byte_offset;

            let dest_ptr = fb.address as *mut u8;

            let src_ptr = dest_ptr.add(byte_offset as usize);

            core::ptr::copy(src_ptr, dest_ptr, bytes_to_copy as usize);


            let clear_y = fb.height - amount_pixels;

            fill_rect(0, clear_y, fb.width, amount_pixels, 0x000000);
        }
    }
}
