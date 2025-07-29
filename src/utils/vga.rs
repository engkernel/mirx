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
        SCREEN_Y_START
    ));
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VGAChar {
    ascii_char: u8,
    color: u8,
}

pub struct VGABuf {
    addr: *mut VGAChar,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl VGABuf {
    pub const fn new(addr: *mut u8, width: usize, height: usize, x: usize, y: usize) -> Self {
        VGABuf {
            addr: addr as *mut VGAChar,
            height,
            width,
            x,
            y,
        }
    }

    fn write_char(&mut self, c: u8, color: u8) {
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

        let index = self.y * self.width + self.x;
        unsafe {
            write_volatile(
                self.addr.add(index),
                VGAChar {
                    ascii_char: c,
                    color,
                },
            );
        }
        self.x += 1;
    }

    pub fn write(&mut self, s: &str) {
        for &byte in s.as_bytes() {
            self.write_char(byte, 0xF);
        }
    }
}

impl fmt::Write for VGABuf {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    VGA.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => ($crate::utils::vga::_print(format_args!($($arg)*)));
}
