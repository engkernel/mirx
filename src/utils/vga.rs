use core::{fmt, ptr::write_volatile};
use lazy_static::lazy_static;
use spin::Mutex;

unsafe impl Send for VGABuf {}
unsafe impl Sync for VGABuf {}

const VGA_ADDRESS: *mut u8 = 0xb8000 as *mut u8;
const SCREEN_WIDTH: usize = 80;
const SCREEN_HEIGHT: usize = 40;
const SCREEN_X_START: usize = 0;
const SCREEN_Y_START: usize = 0;

lazy_static! {
    pub static ref VGA: Mutex<VGABuf> = Mutex::new(VGABuf::new(
        VGA_ADDRESS,
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        SCREEN_X_START,
        SCREEN_Y_START,
        VGAColor::new(Color::White, Color::Black),
    ));
}

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    Blue = 0x1,
    Green = 0x2,
    Cyan = 0x3,
    Red = 0x4,
    Magenta = 0x5,
    Brown = 0x6,
    LightGray = 0x7,
    DarkGray = 0x8,
    LightBlue = 0x9,
    LightGreen = 0xA,
    LightCyan = 0xB,
    LightRed = 0xC,
    LightMagenta = 0xD,
    Yellow = 0xE,
    White = 0xF,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VGAColor(u8);
impl VGAColor {
    pub fn new(foreground: Color, background: Color) -> VGAColor
    {
        VGAColor((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VGAChar {
    ascii_char: u8,
    color: VGAColor,
}

// we do store cursor position as x, y
pub struct VGABuf {
    addr: *mut VGAChar,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    color: VGAColor,
}

impl VGABuf {
    pub const fn new(addr: *mut u8, width: usize, height: usize, x: usize, y: usize, color: VGAColor) -> Self {
        VGABuf {
            addr: addr as *mut VGAChar,
            height,
            width,
            x,
            y,
            color,
        }
    }

    pub fn set_color(&mut self, color: VGAColor) {
        self.color = color
    }

    fn write_char(&mut self, c: u8) {
        if c == b'\n' {
            self.y += 1;
            self.x = 0;
        }

        if c == b'\t' {
            self.y += 4;
        }

        if self.x > self.width {
            self.x = 0;
        }

        if self.y > self.height {
            self.y = 0;
        }

        let index = self.y * self.width + self.x;
        unsafe {
            write_volatile(
                self.addr.add(index),
                VGAChar {
                    ascii_char: c,
                    color: self.color,
                },
            );
        }
        self.x += 1;
    }

    pub fn write(&mut self, s: &str) {
        for &byte in s.as_bytes() {
            self.write_char(byte);
        }
    }

    pub fn clear_scr(&mut self) {
        self.color = VGAColor::new(Color::Black, Color::Black);
        
        // reset vga cursor to zero
        self.x = 0;
        self.y = 0;

        let total_cells = self.height * self.width;
        for _ in 0..total_cells {
            self.write_char(b' ');
        }

        // after clear screen again reset vga cursor to zero
        self.x = 0;
        self.y = 0;
    }
}

impl fmt::Write for VGABuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}