//! A safe interface for accessing and
//! interacting with the `perf_event_open()`
//! and `ioctl()` system calls;
//! and their raw file descriptors.
use crate::event::sys::sys;
use crate::event::sys::wrapper::{ioctl_wrap, read_wrap};
use crate::event::utils::*;

pub type perf_event_attr = sys::perf_event_attr;

/// Stores a raw file descriptor
/// for use in various `perf_event_open()`
/// system call wrappers.
#[derive(Debug)]
pub struct FileDesc(i32);

impl FileDesc {
    /// Set up performance monitoring for
    /// configured event without any flags.
    /// Panics if `perf_event_open()` fails.
    pub fn new(event: &mut perf_event_attr, pid: i32, cpu: i32, group_fd: i32) -> Self {
        let ret: i32;
        ret = sys::perf_event_open(event, pid, cpu, group_fd, 0) as i32;
        if ret == -1 {
            panic!("Panic: system call perf_event_open() failed in PerfEventFd::new()");
        }
        Self(ret)
    }

    /// Enable the performance counter
    /// associated with `fd`.
    pub fn enable(&self) -> Result<(), SysErr> {
        if ioctl_wrap(self.0, sys::ENABLE, 0) == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }

    /// Disable the performance counter
    /// associated with `fd`.
    pub fn disable(&self) -> Result<(), SysErr> {
        if ioctl_wrap(self.0, sys::DISABLE, 0) == -1 {
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
    pub fn refresh(&self, count: u64) -> Result<(), SysErr> {
        // passing an argument of 0
        // along with `sys::REFRESH`
        // introduces undefined behavior.
        if count == 0 {
            return Err(SysErr::IoArg);
        }
        let arg: *const u64 = &count;
        if ioctl_wrap(self.0, sys::REFRESH, arg) == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }

    /// Reset the performance counter to 0.
    pub fn reset(&self) -> Result<(), SysErr> {
        if ioctl_wrap(self.0, sys::RESET, 0) == -1 {
            return Err(SysErr::IoFail);
        }
        Ok(())
    }

    /// Set the overflow period.
    /// The interval argument to the
    /// `ioctl()` must be a pointer to
    /// an unsigned 64-bit integer.
    /// NOTE: The `__bindgen_anon_1` and `sample_type` fields
    /// must be initialized for the `perf_event_attr`
    /// struct that is passed to `FileDesc::new()`.
    pub fn overflow_period(&self, interval: u64) -> Result<(), SysErr> {
        let arg: *const u64 = &interval;
        if ioctl_wrap(self.0, sys::PERIOD, arg) == -1 {
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
    pub fn id(&self) -> Result<u64, SysErr> {
        // Write event id value
        // to location specified by arg.
        let mut ret: u64 = 0;
        let arg: *mut u64 = &mut ret;
        if ioctl_wrap(self.0, sys::ID, arg) == -1 {
            return Err(SysErr::IoFail);
        }
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

    pub fn read(&self) -> Result<isize, SysErr> {
        let ret = read_wrap(self.0);
        if ret == -1 {
            return Err(SysErr::ReadFail);
        }
        Ok(ret)
    }
}

#[cfg(test)]
#[test]
fn interface_test() {
    let sample_struct = sys::perf_event_attr__bindgen_ty_1 { sample_period: 1 };
    let event = &mut perf_event_attr {
        type_: sys::perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf_event_attr>() as u32,
        config: sys::perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
        __bindgen_anon_1: sample_struct,
        sample_type: sys::perf_event_sample_format_PERF_SAMPLE_IP,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    // Panic on failure.
    let fd = FileDesc::new(event, 0, -1, -1);
    // Make sure ioctls are working.
    fd.reset().unwrap();
    fd.disable().unwrap();
    fd.enable().unwrap();
    let cnt: isize = fd.read().unwrap();
    fd.id().unwrap();
    // change overflow sampling period
    fd.overflow_period(2).unwrap();
    fd.refresh(3).unwrap();
    assert_ne!(cnt, 0);
    assert!(cnt > 0, "cnt = {}", cnt);
}
