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
include!("../bindings/perf_event.rs");

extern crate libc;

use libc::{c_int, c_ulong, ioctl, pid_t, read, syscall, SYS_perf_event_open};

mod constants;

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

#[cfg(test)]
#[test]
fn syscall_test() {
    let event = &mut perf_event_attr {
        type_: perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf_event_attr>() as u32,
        // something to consider fixing. For now leave alone.
        config: perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    let fd: isize;
    fd = perf_event_open(&event, 0, -1, -1, 0);
    assert_ne!(fd, -1, "Testing for failure");
    assert_eq!(0, unsafe {
        ioctl(fd as i32, constants::RESET as u64, 0)
    });
}
#[test]
fn read_test() {
    let event = &mut perf_event_attr {
        type_: perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf_event_attr>() as u32,
        // something to consider fixing. For now leave alone.
        config: perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    let fd: isize;
    fd = perf_event_open(&event, 0, -1, -1, 0);
    //read treats each counter as virtualized u64
    let mut cnt: u64 = 0;
    //buf must be *mut lbc::c_void type, mimics void pointer
    //package count into buf so it is easy to read
    let buf: *mut libc::c_void = &mut cnt as *mut _ as *mut libc::c_void;
    unsafe {
        ioctl(fd as i32, constants::ENABLE as u64, 0);
        read(fd as i32, buf, std::mem::size_of_val(&cnt));
    }
    assert_ne!(cnt, 0);
    assert!(cnt > 0, "cnt = {}", cnt);
}
