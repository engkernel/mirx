#![no_std]
#![no_main]

mod utils;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    panic!("{}", _info);
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    info!("This is info {}", 46);
    debug!("This is {}", 46);
    error!("This is error {}", 34);
    panic!("This is panic {}!!", 23);
}
