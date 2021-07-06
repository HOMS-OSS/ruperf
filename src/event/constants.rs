//! Rust implementations of `_IO()` macros
//! defined in `/usr/include/asm-generic/ioctl.h`.
//! Note that some constant values defined
//! here vary by architecture, and that these constants
//! are produced by `bindgen`. These macros are used to
//! define the constants for use in perf-related ioctls.

include!("../bindings/perf_event.rs");

macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => {
        (($dir as u32) << _IOC_DIRSHIFT)
            | (($ty as u32) << _IOC_TYPESHIFT)
            | (($nr as u32) << _IOC_NRSHIFT)
            | (($sz as u32) << _IOC_SIZESHIFT)
    };
}

macro_rules! io {
    ($ty:expr, $nr:expr) => {
        ioc!(_IOC_NONE, $ty, $nr, 0)
    };
}

macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(_IOC_READ, $ty, $nr, $sz)
    };
}

macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(_IOC_WRITE, $ty, $nr, $sz)
    };
}

macro_rules! iowr {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(_IOC_READ | _IOC_WRITE, $ty, $nr, $sz)
    };
}

///
pub const PERF_EVENT_IOC_ENABLE: u32 = io!(b'$', 0);
pub const PERF_EVENT_IOC_DISABLE: u32 = io!(b'$', 0);
pub const PERF_EVENT_IOC_REFRESH: u32 = io!(b'$', 2);
pub const PERF_EVENT_IOC_RESET: u32 = io!(b'$', 3);
pub const PERF_EVENT_IOC_PERIOD: u32 = iow!('$', 4, std::mem::size_of::<libc::__u64>());
pub const PERF_EVENT_IOC_SET_OUTPUT: u32 = io!(b'$', 5);
pub const PERF_EVENT_IOC_SET_FILTER: u32 = iow!(b'$', 6, std::mem::size_of::<*const char>());
pub const PERF_EVENT_IOC_ID: u32 = ior!(b'$', 7, std::mem::size_of::<*const u64>());
pub const PERF_EVENT_IOC_SET_BPF: u32 = iow!(b'$', 8, std::mem::size_of::<libc::__u32>());
pub const PERF_EVENT_IOC_PAUSE_OUTPUT: u32 = iow!(b'$', 9, std::mem::size_of::<libc::__u32>());
pub const PERF_EVENT_IOC_QUERY_BPF: u32 =
    iowr!(b'$', 10, std::mem::size_of::<*const perf_event_query_bpf>());
pub const PERF_EVENT_IOC_MODIFY_ATTRIBUTES: u32 =
    iowr!(b'$', 11, std::mem::size_of::<*const perf_event_attr>());
