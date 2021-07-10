//! A `FileDesc` provides a safe interface for
//! accessing and interacting with the `perf_event_open()`
//! and `ioctl()` system calls, and their raw file descriptors.
//!
//! A wrapper is not provided for the `perf_event_open()` system call.
//! Necessitating the use of `unsafe { syscall(..) }`.
//! See linux man-page NOTES for details.
include!("../../bindings/perf_event.rs");

extern crate libc;

use libc::{c_int, c_ulong, ioctl, pid_t, read, syscall, SYS_perf_event_open};

mod constants;

/// Result type for anytime
/// `ioctl()` returns -1.
// TODO: get value of `errno`
// for more accurate error handling.
type Result<T> = std::result::Result<T, IoError>;

/// This error type should
/// eventually be used to
/// provide information
/// based on value of `errno`.
#[derive(Debug)]
pub enum IoError {
    SysCallFail,
    InvalidArg,
    InvalidId,
}

/// Stores a raw file descriptor
/// for use in various `perf_event_open()`
/// system call wrappers.
#[derive(Debug)]
pub struct FileDesc(i32);

impl FileDesc {
    /// Set up performance monitoring for
    /// configured event without any flags.
    /// Panics if `perf_event_open()` fails.
    ///
    /// # examples
    /// ```
    ///	let fd = PerfEventFd::new(event, 0, -1, -1);
    /// ```
    pub fn new(event: &mut perf_event_attr, pid: i32, cpu: i32, group_fd: i32) -> Self {
        let ret: i32;
        ret = perf_event_open(event, pid as pid_t, cpu, group_fd, 0) as i32;
        if ret == -1 {
            panic!("Panic: system call perf_event_open() failed in PerfEventFd::new()");
        }
        Self(ret)
    }

    /// Enable the performance counter
    /// associated with `fd`.
    pub fn enable(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::ENABLE as u64, 0) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Disable the performance counter
    /// associated with `fd`.
    pub fn disable(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::DISABLE as u64, 0) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Refresh the overflow counter.
    /// `count` is added to a register
    /// that is decremented each time
    /// the counter for the event associated
    /// with `fd` overflows. When the counter
    /// reaches 0, the event is disabled.
    pub fn refresh(&self, arg: u32) -> Result<()> {
        let ret: i32;
        // passing an argument of 0
        // with this ioctl is undefined behavior.
        if arg < 0 {
            return Err(IoError::InvalidArg);
        }
        ret = unsafe { libc::ioctl(self.0, constants::REFRESH as u64, arg) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Reset the performance counter to 0.
    pub fn reset(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::RESET as u64, 0) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Set the overflow period.
    /// The interval argument to the
    /// `ioctl()` must be a pointer to
    /// an unsinged 64-bit integer.
    pub fn overflow_period(&self, interval: *const u64) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::PERIOD as u64, interval) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Report counter information to
    /// specific file descriptor.
    pub fn set_output(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::SET_OUTPUT as u64, 0) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Ignore counter output for event
    /// associated with `fd`.
    pub fn ignore_output(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::SET_OUTPUT as u64, -1) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Return event ID value
    /// associated with `fd`.
    pub fn id(&self) -> Result<u64> {
        // forgive me father.
        let mut ret: u64 = 0;
        ret = unsafe {
            let result: *mut u64 = &mut ret;
            if libc::ioctl(self.0, constants::ID as u64, result) == -1 {
                return Err(IoError::SysCallFail);
            }
            *result
        };
        if ret == 0 {
            return Err(IoError::InvalidId);
        }
        Ok(ret)
    }

    /// Pause writing to ring-buffer
    /// for associated file descriptor.
    pub fn pause_output(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::PAUSE_OUTPUT as u64, 1) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Resume writing to ring-buffer
    /// for associated file descriptor.
    pub fn resume_output(&self) -> Result<()> {
        let ret: i32;
        ret = unsafe { libc::ioctl(self.0, constants::PAUSE_OUTPUT as u64, 0) };
        if ret == -1 {
            return Err(IoError::SysCallFail);
        }
        Ok(())
    }

    /// Modify the attributes for
    /// a specified event.
    pub fn modify_attributes(&self, _event: *const perf_event_attr) -> Result<()> {
        todo!()
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
