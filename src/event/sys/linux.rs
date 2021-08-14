//! SPDX-License-Identifier: GPL-2.0
//! Rust implementations of `_IO()` macros
//! defined in `/usr/include/asm-generic/ioctl.h`.
//! Note that some constant values defined
//! here vary by architecture, and that these constants
//! are produced by `bindgen`. These macros are used to
//! define the constants for use in perf-related ioctls.
use crate::bindings::*;
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
