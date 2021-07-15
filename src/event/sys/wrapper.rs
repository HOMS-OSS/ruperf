extern crate libc;
use libc::{c_void, read};

pub fn read_wrap(fd: i32, mut count: isize) -> isize {
    unsafe {
        let buf: *mut libc::c_void = &mut count as *mut _ as *mut libc::c_void;
        read(fd, buf, std::mem::size_of_val(&count));
    }
    count
}
#[cfg(test)]
#[test]
fn read_wrapper_test() {
    use crate::event::fd;
    use crate::event::sys::*;
    use libc::ioctl;
    let event = &mut fd::perf_event_attr {
        type_: fd::perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<fd::perf_event_attr>() as u32,
        // something to consider fixing. For now leave alone.
        config: fd::perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    let fd = fd::FileDesc::new(event, 0, -1, -1);
    //read treats each counter as virtualized u64
    let mut cnt: isize = 0;
    //buf must be *mut lbc::c_void type, mimics void pointer
    //package count into buf so it is easy to read
    unsafe {
        ioctl(fd.0, sys::ENABLE as u64, 0);
    }
    cnt = read_wrap(fd.0, cnt);
    assert_ne!(cnt, 0);
    assert!(cnt > 0, "cnt = {}", cnt);
}
