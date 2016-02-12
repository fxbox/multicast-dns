extern crate libc;

use std::mem;

use libc::{c_int, c_void};

extern {
    fn do_something(f: Option<extern fn (x: c_int, arg: *mut c_void) -> c_int>, arg: *mut c_void) -> c_int;
}

extern fn do_something_handler(x: c_int, arg: *mut c_void) -> c_int {
    let closure: &mut &mut FnMut(i32) -> bool = unsafe { mem::transmute(arg) };
    closure(x as i32) as c_int
}

pub fn do_with_callback<F>(x: i32, mut callback: F) -> bool where F: FnMut(i32) -> bool {
    let cb: &mut FnMut(i32) -> bool = &mut callback;
    unsafe {
        do_something(Some(do_something_handler), cb as *mut _ as *mut c_void) > 0
    }
}