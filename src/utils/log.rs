use core::fmt;
use crate::utils::vga::{Color, VGAColor, VGA};

// logs management and levels
pub enum LogLevel {
    Debug,
    Info,
    Error,
    Panic,
}

impl LogLevel {
    pub fn get_level(&self) -> VGAColor {
        match self {
            LogLevel::Debug => VGAColor::new(Color::Yellow, Color::Black),
            LogLevel::Info => VGAColor::new(Color::LightGray, Color::Black),
            LogLevel::Error => VGAColor::new(Color::LightRed, Color::Black),
            LogLevel::Panic => VGAColor::new(Color::White, Color::Red),
        }
    }
}

#[doc(hidden)]
pub fn _log(level: VGAColor, args: fmt::Arguments) {
    use core::fmt::Write;
    let mut vga = VGA.lock();
    vga.set_color(level);
    vga.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::utils::log::_log($level.get_level(), format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    () => {
        $crate::log!($crate::utils::log::LogLevel::Debug, "\n")
    };
    ($($arg:tt)*) => {{ $crate::log!($crate::utils::log::LogLevel::Debug, $($arg)*)}};
}

#[macro_export]
macro_rules! info {
    () => {
        log!($crate::utils::log::LogLevel::Info, "\n")
    };
    ($($arg:tt)*) => {{ log!($crate::utils::log::LogLevel::Info, $($arg)*)}};
}

#[macro_export]
macro_rules! error {
    () => {
        log!($crate::utils::log::LogLevel::Error, "\n")
    };
    ($($arg:tt)*) => {{ log!($crate::utils::log::LogLevel::Error, $($arg)*)}};
}

#[macro_export]
macro_rules! panic {
    () => {
        log!($crate::utils::log::LogLevel::Panic, "\n")
    };
    ($($arg:tt)*) => {{ 
        log!($crate::utils::log::LogLevel::Panic, $($arg)*);
        loop{};
    }};
}

// clear screen
#[doc(hidden)]
pub fn _clear_scr() {
    let mut vga = VGA.lock();
    vga.clear_scr();
}

#[macro_export]
macro_rules! clear {
    () => {
        $crate::utils::log::_clear_scr();
    };
}