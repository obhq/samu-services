#![no_std]
#![no_main]

use self::module::load_module;
use core::ffi::{c_int, CStr};
use core::panic::PanicInfo;

mod module;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> c_int {
    // Load libkernel.
    let libkernel = match load_libkernel() {
        Ok(v) => v,
        Err(_) => {
            // We can't do anything here even printing the error.
            return 1;
        }
    };

    0
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

fn load_libkernel() -> Result<c_int, crate::syscall::Error> {
    if let Ok(v) = load_module(CStr::from_bytes_with_nul(b"libkernel.sprx\0").unwrap()) {
        return Ok(v);
    }

    if let Ok(v) = load_module(CStr::from_bytes_with_nul(b"libkernel_web.sprx\0").unwrap()) {
        return Ok(v);
    }

    load_module(CStr::from_bytes_with_nul(b"libkernel_sys.sprx\0").unwrap())
}
