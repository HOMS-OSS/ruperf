//! `ioctl()` wrapper/interface 
//! restricting system call access.
//! 
//! For more documentation on the `ioctl()`
//! system call, see the Linux man page. 
//!
//! `ioctl()` poses a security risk due 
//! it's sensitive nature and variable number of arguments.
//! To mitigate this, these functions provide an interface
//! that restricts the number of arguments to 
//! only those necessary for specific `perf_event_open()` ioctls.
extern crate libc;

/// Result type for anytime
/// `ioctl()` returns -1.
//!
//! TODO: get value of `errno` 
//! for more accurate error handling.
type Result<T> = std::result::Result<T, IoError>;

/// This error type should
/// eventually be used to 
/// provide more specific information 
/// depending on the value of `errno`.
#[derive(Debug)]
pub enum IoError {
	SysCallFail,
	InvalidArg,
}

//! TODO: add lifetimes for functions, file descriptors.

/// Enable the performance counter
/// associated with `fd`.
pub fn enable(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::ENABLE, 0) }
	if ret == -1 {
		return Err(IoError::SysCallFail)
	}
	Ok(())
}

/// Disable the performance counter
/// associated with `fd`.
pub fn disable(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::DISABLE, 0) }
	if ret == -1 {
		return Err(IoError::SysCallFail)
	}
	Ok(())
}

/// Refresh the overflow counter.
/// `count` is added to a register
/// that is decremented each time
/// the counter for the event associated 
/// with `fd` overflows. When the counter
/// reaches 0, the event is disabled.
pub fn refresh(fd: i32, count: u32) -> Result (()) {
	let ret: i32;
	// passing an argument of 0
	// with this ioctl is undefined behavior.
	if count < 0 {
		return Err(IoError::InvalidArg)
	}
	ret = unsafe { libc::ioctl(fd, constants::REFRESH, count) }
	if ret == -1 {
		return Err(IoError::SysCallFail)
	}
	Ok(())
}

/// Reset the performance counter to 0.
pub fn reset(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::RESET, 0) }
	if ret == -1 { 
		return Err(IoError::SysCallFail);
	}
	Ok(())
}

/// Set the overflow period.
/// The interval argument to the
/// `ioctl()` must be a pointer to 
/// an unsinged 64-bit integer.
pub fn overflow_period<'a>(fd: i32, interval: *const 'a u64) -> Result() {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::PERIOD, interval) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	Ok(())
}

/// Report counter information to 
/// specific file descriptor.
pub fn set_output(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::SET_OUTPUT, 0) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	Ok(())
}

/// Ignore counter output for event
/// associated with `fd`.
pub fn ignore_output(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::SET_OUTPUT, -1) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	Ok(())
}

/// Add ftrace filter to event
/// associated with `fd`.
pub fn set_filter(fd: i32) -> Result(()) {
	todo!()
}

/// Return event ID value
/// associated with `fd`.
pub fn id(fd: i32) -> Result(u64) {
	let ret: i32;
	// `ioctl()` writes the result of the
	// `PERF_EVENT_IOC_ID` command to 
	let result: *mut u64;
	ret = unsafe { libc::ioctl(fd, constants::ID, result) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	let value = unsafe { *result }
	Ok(value)
}

/// Attach bpf program to existing kprobe
/// tracepoint event. `bpfd` must be a BPF
/// program file descriptor created by a 
/// previous call to the `bpf(2)` system call.
pub fn set_bpf(fd: i32, bpfd: i32) -> Result(()) {
	todo!()
}

/// Pause writing to ring-buffer
/// for associated file descriptor.
pub fn pause_output(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::PAUSE_OUTPUT, 1) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	Ok(())
}

/// Resume writing to ring-buffer
/// for associated file descriptor.
pub fn resume_output(fd: i32) -> Result(()) {
	let ret: i32;
	ret = unsafe { libc::ioctl(fd, constants::PAUSE_OUTPUT, 0) }
	if ret == -1 {
		return Err(SysCallFail)
	}
	Ok(())
}

pub fn query_bpf<'a>(fd: i32, event_query: *const <'a> perf_event_query_bpf) -> Result(()) {
	todo!()
}
pub fn modify_attributes<'a>(fd: i32, event: *const <'a> perf_event_attr) -> Result(()) {
	todo!()
}

#[cfg(test)]
#[test]
fn test_ioctl_wrappers() {
	todo!()
}
