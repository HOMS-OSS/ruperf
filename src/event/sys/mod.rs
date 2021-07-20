//! This module and it's sub-modules
//! serve as a sort of 'dungeon'
//! where all the unsafe code goes.
pub mod sys;
pub mod wrapper;

#[cfg(test)]
#[test]
fn read_test() {
    let event = &mut sys::perf_event_attr {
        type_: sys::perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<sys::perf_event_attr>() as u32,
        // something to consider fixing. For now leave alone.
        config: sys::perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    let fd: isize;
    fd = sys::perf_event_open(&event, 0, -1, -1, 0);
    unsafe {
        libc::ioctl(fd as i32, sys::ENABLE as u64, 0);
    }
    let cnt = wrapper::read_wrap(fd as i32);
    assert_ne!(cnt, 0);
    assert!(cnt > 0, "cnt = {}", cnt);
}
