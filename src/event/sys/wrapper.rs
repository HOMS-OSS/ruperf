//! SPDX-License-Identifier: GPL-2.0
//! A safe wrapper for the `read()`
//! Linux system call. For more on
//! `read()` see the Linux man-page.

extern crate libc;
use libc::read;

pub fn read_wrap(fd: i32) -> isize {
    let mut count: isize = 0;
    unsafe {
        let buf: *mut libc::c_void = &mut count as *mut _ as *mut libc::c_void;
        read(fd, buf, std::mem::size_of_val(&count));
    }
    count
}
