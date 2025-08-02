#![no_std]
#![no_main]

mod utils;
mod arch;
use core::panic::PanicInfo;
use bootloader::BootInfo;

use crate::arch::x86::{self, kernel::init};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic!("{}", _info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    init::x86_init();
    loop{};
}
