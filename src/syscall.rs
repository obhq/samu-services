use core::ffi::c_int;

// All syscall function return negative errno in case of error.
#[naked_function::naked]
pub unsafe extern "C" fn syscall4(id: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> i32 {
    asm!("syscall", "ret");
}

/// Encapsulte an error from a syscall.
pub struct Error {
    errno: c_int,
}

impl Error {
    /// `ret` must be a negative errno otherwise this function will panic.
    pub fn new(ret: i32) -> Self {
        if ret >= 0 {
            panic!("The specified value is non-negative.");
        }

        Self { errno: ret.abs() }
    }
}
