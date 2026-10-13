use core::fmt::Write;
use spin::Mutex;
use crate::graphics::{FB_GLOBAL};
use font8x8::{BASIC_FONTS, UnicodeFonts};
use crate::graphics;

#[used]
pub static CONSOLE: Mutex<Console> = Mutex::new(Console {
    x: 0,
    y: 0,
    color: 0xFFFFFF,
    scale: 2,
});

pub struct Console {
    pub x: u64,
    pub y: u64,
    pub color: u32,
    pub scale: u64,
}
impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            unsafe {
                if let Some(ref fb) = FB_GLOBAL {
                    let line_height = 10 * self.scale;

                    if c == '\n' {
                        self.x = 0;
                        if self.y + line_height >= fb.height {
                            graphics::scroll_down(line_height);
                        } else {
                            self.y += line_height;
                        }
                        continue;
                    }

                    if self.x + 8 * self.scale > fb.width {
                        self.x = 0;
                        if self.y + line_height >= fb.height {
                            graphics::scroll_down(line_height);
                        } else {
                            self.y += line_height;
                        }
                    }

                    if let Some(glyph_data) = BASIC_FONTS.get(c) {
                        let glyph: [u8; 8] = glyph_data;
                        graphics::print_letters(self.x, self.y, self.color, self.scale, glyph);
                    }

                    self.x += 8 * self.scale;
                }
            }
        }
        Ok(())
    }
}