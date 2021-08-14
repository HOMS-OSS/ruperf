//! SPDX-License-Identifier: GPL-2.0
//! A `FileDesc` provides a safe interface for
//! accessing and interacting with the `perf_event_open()`
//! and `ioctl()` system calls, and their raw file descriptors.
//!
//! A wrapper is not provided for the `perf_event_open()` system call.
//! Necessitating the use of `unsafe { syscall(..) }`.
//! See linux man-page NOTES for details.

extern crate libc;
use crate::bindings::*;
use crate::event::sys::linux::*;
use crate::event::sys::wrapper::*;
use crate::event::utils::*;
use libc::{c_int, c_ulong, pid_t, syscall, SYS_perf_event_open};

/// Stores a raw file descriptor
/// for use in various `perf_event_open()`
/// system call wrappers.
#[derive(Debug)]
pub struct FileDesc(i32);

impl FileDesc {
    /// Set up performance monitoring for
    /// configured event without any flags.
    /// Panics if `perf_event_open()` fails.
    pub fn new(event: &mut perf_event_attr, pid: Option<i32>, cpu: i32, group_fd: i32) -> Self {
        let ret: i32;
        let pid = match pid {
            Some(x) => x as pid_t,
            None => 0_i32,
        };
        ret = perf_event_open(event, pid as pid_t, cpu, group_fd, 0) as i32;
        if ret == -1 {
            panic!("Panic: system call perf_event_open() failed in FileDesc::new()");
        }
        Self(ret)
    }
    /// Enable the performance counter
    /// associated with `fd`.
    pub fn enable(&self) -> Result<(), SysErr> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, ENABLE as u64, 0) };
        if ret == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }
    /// Disable the performance counter
    /// associated with `fd`.
    pub fn disable(&self) -> Result<(), SysErr> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, DISABLE as u64, 0) };
        if ret == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }
    /// Refresh the overflow counter.
    /// `count` is added to a register
    /// that is decremented each time
    /// the counter for the event associated
    /// with `fd` overflows. When the counter
    /// reaches 0, the event is disabled.
    pub fn refresh(&self, count: usize) -> Result<(), SysErr> {
        let ret: i32;
        // passing an argument of 0
        // with this ioctl is undefined behavior.
        if count == 0 {
            return Err(SysErr::IoArg);
        }
        let arg: *const usize = &count;
        ret = unsafe { libc::ioctl(self.0, REFRESH as u64, arg) };
        if ret == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }
    /// Reset the performance counter to 0.
    pub fn reset(&self) -> Result<(), SysErr> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, RESET as u64, 0) };
        if ret == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }
    /// Set the overflow period.
    /// NOTE: The `__bindgen_anon_1` and `sample_type` fields
    /// must be initialized for the `perf_event_attr`
    /// struct that is passed to `FileDesc::new()`.
    pub fn overflow_period(&self, interval: usize) -> Result<(), SysErr> {
        let ret: i32;
        let arg: *const usize = &interval;
        ret = unsafe { libc::ioctl(self.0, PERIOD as u64, arg) };
        if ret == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }
    /// Report counter information to
    /// specific file descriptor.
    pub fn set_output(&self) -> Result<(), SysErr> {
        todo!()
    }
    /// Ignore counter output for event
    /// associated with `fd`.
    pub fn ignore_output(&self) -> Result<(), SysErr> {
        todo!()
    }
    /// Return event ID value
    /// associated with `fd`.
    pub fn id(&self) -> Result<usize, SysErr> {
        // forgive me father.
        let mut ret: usize = 0;
        ret = unsafe {
            let result: *mut usize = &mut ret;
            if libc::ioctl(self.0, ID as u64, result) == -1 {
                return Err(SysErr::IoFail);
            }
            *result
        };
        if ret == 0 {
            return Err(SysErr::IoId);
        }
        Ok(ret)
    }
    /// Pause writing to ring-buffer
    /// for associated file descriptor.
    pub fn pause_output(&self) -> Result<(), SysErr> {
        todo!()
    }
    /// Resume writing to ring-buffer
    /// for associated file descriptor.
    pub fn resume_output(&self) -> Result<(), SysErr> {
        todo!()
    }
    /// Modify the attributes for
    /// a specified event.
    pub fn modify_attributes(&self, _event: *const perf_event_attr) -> Result<(), SysErr> {
        todo!()
    }
    /// Read counter value associated
    /// with field of `FileDesc` caller.
    pub fn read(&self) -> Result<isize, SysErr> {
        let ret = read_wrap(self.0);
        if ret == -1 {
            return Err(SysErr::ReadFail);
        }
        Ok(ret)
    }
}

/// For documentation on `perf_event_open()`
/// system call, see the Linux man page.
fn perf_event_open(
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

#[test]
fn read_test() {
    use libc::{ioctl, read};
    let event = &mut perf_event_attr {
        type_: perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf_event_attr>() as u32,
        // Something to consider fixing. For now leave alone.
        config: perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    let fd: isize;
    fd = perf_event_open(&event, 0, -1, -1, 0);
    // Read treats each counter as a virtualized u64.
    let mut cnt: u64 = 0;
    // `buf` must be *mut lbc::c_void type, mimics void pointer.
    // Package count into buf so it is easy to read.
    let buf: *mut libc::c_void = &mut cnt as *mut _ as *mut libc::c_void;
    unsafe {
        ioctl(fd as i32, ENABLE as u64, 0);
        read(fd as i32, buf, std::mem::size_of_val(&cnt));
    }
    assert_ne!(cnt, 0);
    assert!(cnt > 0, "cnt = {}", cnt);
}
