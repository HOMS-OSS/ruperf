//! This module contains the various `ioctl()`
//! commands related to performance monitoring;
//! and a wrapper for the `perf_event_open()` system call.
//!
//! Note that the constants defined in this file
//! vary by architecture.
include!("../../bindings/perf_event.rs");
use std::mem::size_of;

/// '$' is the ioctl number for `linux/perf_event`.
/// For more information on ioctl numbers
/// see `Linux ioctls' reference in `README.md`.
const _IO_TYPE: u32 = b'$' as u32;

/// Neither read nor write to kernel
const fn iocn(nr: u32) -> u32 {
    // no direction,
    // no size.
    let sz: usize = 0;
    (_IOC_NONE << _IOC_DIRSHIFT)
        | (_IO_TYPE << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((sz as u32) << _IOC_SIZESHIFT)
}

/// User: write
/// Kernel: read
const fn iocw(nr: u32, sz: usize) -> u32 {
    (_IOC_WRITE << _IOC_DIRSHIFT)
        | (_IO_TYPE << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((sz as u32) << _IOC_SIZESHIFT)
}

/// User: read
/// Kernel: write
const fn iocr(nr: u32, sz: usize) -> u32 {
    (_IOC_READ << _IOC_DIRSHIFT)
        | (_IO_TYPE << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((sz as u32) << _IOC_SIZESHIFT)
}

/// Kernel: reads and writes
const fn iocwr(nr: u32, sz: usize) -> u32 {
    ((_IOC_READ | _IOC_WRITE) << _IOC_DIRSHIFT)
        | (_IO_TYPE << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | ((sz as u32) << _IOC_SIZESHIFT)
}

pub const ENABLE: u32 = iocn(0);
pub const DISABLE: u32 = iocn(1);
pub const REFRESH: u32 = iocn(2);
pub const RESET: u32 = iocn(3);
pub const PERIOD: u32 = iocw(4, size_of::<u64>());
pub const SET_OUTPUT: u32 = iocn(5);
/** NOT SUPPORTED **/
pub const SET_FILTER: u32 = iocw(6, size_of::<*const char>());
pub const ID: u32 = iocr(7, size_of::<*const u64>());
/** NOT SUPPORTED **/
pub const SET_BPF: u32 = iocw(8, size_of::<u32>());
pub const PAUSE_OUTPUT: u32 = iocw(9, size_of::<u32>());
/** NOT SUPPORTED **/
pub const QUERY_BPF: u32 = iocwr(10, size_of::<*const perf_event_query_bpf>());
pub const MODIFY_ATTRIBUTES: u32 = iocwr(11, size_of::<*const perf_event_attr>());

/// For documentation on `perf_event_open()`
/// system call, see the Linux man page.
pub fn perf_event_open(
    event: &perf_event_attr,
    pid: i32,
    cpu: i32,
    group_fd: i32,
    flags: usize,
) -> isize {
    unsafe {
        libc::syscall(
            libc::SYS_perf_event_open,
            event,
            pid as libc::pid_t,
            cpu as libc::c_int,
            group_fd as libc::c_int,
            flags as libc::c_ulong,
        ) as isize
    }
}

#[cfg(test)]
#[test]
fn perf_event_open_test() {
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
}
