extern crate libc;

use libc::c_void;

struct Userdata {
    callback: Box<Fn(i32)>,
}

#[link(name = "asynchronous_callbacks")]
extern {
    fn c_perform_task(value: i32, callback: unsafe extern "C" fn(*mut c_void, i32), userdata: *mut c_void);
}

unsafe extern fn callback(userdata: *mut c_void, result: i32) {
    (Box::from_raw(userdata as *mut Userdata).callback)(result);
}

fn perform_task<F: Fn(i32) + 'static>(value: i32, f: F) {
    let userdata = Box::into_raw(Box::new(Userdata { callback: Box::new(f) })) as *mut c_void;
    unsafe { c_perform_task(value, callback, userdata) }
}

fn main() {
    println!("Performing task...");

    perform_task(5, |result| println!("Result: {}", result));
}
