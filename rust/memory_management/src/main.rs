extern crate libc;

use libc::c_char;
use std::ffi::CString;

#[link(name = "memory_management")]
extern {
	fn string_length(
		string: *const c_char) -> i32;
}

fn main() {
    let c_string = CString::new("Hello, world!").unwrap();

	println!("length: {}",
		unsafe { string_length(c_string.as_ptr()) });
}
