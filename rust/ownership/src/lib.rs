extern crate libc;

use libc::c_char;
use std::ffi::CString;

#[no_mangle]
pub unsafe extern fn stringify(x: i32) -> *mut c_char {
    CString::new(format!("x is equal to {}", x))
        .unwrap()
        .into_raw()
}

#[no_mangle]
pub unsafe extern fn release_string(string: *mut c_char) {
    CString::from_raw(string);
}
