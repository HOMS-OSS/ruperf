//! Wrapper for `perf_event_open()` system call.
//!
//! A wrapper is not provided for the `perf_event_open()` system call.
//! Necessitating the use of `unsafe { syscall(..) }`.
//! See linux man-page NOTES for details.
//!
//! For notes about Rust bindings necessary for
//! `perf_event_open()` see /src/bindings/perf_event.rs.

//! Disable cargo build warnings created due to using bindgen
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!("bindings/perf_event.rs");

extern crate libc;

use libc::{c_int, c_ulong, pid_t, syscall, SYS_perf_event_open};

pub fn perf_event_open(
    event: &perf_event_attr,
    pid: pid_t,
    cpu: i32,
    group_fd: i32,
    flags: usize,
) -> isize {
    unsafe {
        syscall(
            SYS_perf_event_open,
            event,
            pid,
            cpu as c_int,
            group_fd as c_int,
            flags as c_ulong,
        ) as isize
    }
}

pub fn perf_event_hello() {
    println!("hello from your friendly perf_event file");
}
