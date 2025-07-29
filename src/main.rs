#![no_std]
#![no_main]

mod utils;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // info!();
    debug!("This is {}", 46);
    // err!();
    // panic!();
    loop {}
}
