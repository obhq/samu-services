use crate::syscall::{syscall4, Error};
use core::ffi::{c_int, CStr};

pub fn load_module<N: AsRef<CStr>>(name: N) -> Result<c_int, Error> {
    let mut id: c_int = 0;
    let ret = unsafe {
        syscall4(
            594,
            name.as_ref().as_ptr() as i64,
            0,
            &mut id as *mut c_int as i64,
            0,
        )
    };

    if ret < 0 {
        Err(Error::new(ret))
    } else {
        Ok(id)
    }
}
