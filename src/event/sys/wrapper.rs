extern crate libc;
use libc::{c_void, read};

pub fn read_wrap(fd: i32) -> isize {
    let mut count: isize = 0;
    unsafe {
        let buf: *mut libc::c_void = &mut count as *mut _ as *mut libc::c_void;
        read(fd, buf, std::mem::size_of_val(&count));
    }
    count
}
