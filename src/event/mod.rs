//! For notes about Rust bindings necessary for
//! `perf_event_open()` see /src/bindings/perf_event.rs.

//! Disable cargo build warnings created due to using bindgen
pub mod event;
mod fd;
mod sys;
mod utils;

pub fn perf_event_hello() {
    println!("hello from your friendly perf_event file");
}

#[cfg(test)]
#[test]
fn wrapper_test() {
    use crate::bindings::*;
    let sample_struct = perf_event_attr__bindgen_ty_1 { sample_period: 1 };
    let event = &mut perf_event_attr {
        type_: perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf_event_attr>() as u32,
        config: perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
        __bindgen_anon_1: sample_struct,
        sample_type: perf_event_sample_format_PERF_SAMPLE_IP,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    // Panic on failure.
    let fd = fd::FileDesc::new(event, 0, -1, -1);
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
