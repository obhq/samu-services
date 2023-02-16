#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> i32 {
    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
