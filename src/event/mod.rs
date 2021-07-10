//! For notes about Rust bindings necessary for
//! `perf_event_open()` see /src/bindings/perf_event.rs.

//! Disable cargo build warnings created due to using bindgen
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

mod perf;

pub fn perf_event_hello() {
    println!("hello from your friendly perf_event file");
}

#[cfg(test)]
#[test]
fn wrapper_test() {
    let event = &mut perf::perf_event_attr {
        type_: perf::perf_type_id_PERF_TYPE_HARDWARE,
        size: std::mem::size_of::<perf::perf_event_attr>() as u32,
        config: perf::perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
        ..Default::default()
    };
    event.set_disabled(1);
    event.set_exclude_kernel(1);
    event.set_exclude_hv(1);
    // Panic on failure.
    let fd = perf::FileDesc::new(event, 0, -1, -1);
    // Make sure ioctls are working.
    fd.reset().unwrap();
    fd.disable().unwrap();
    fd.enable().unwrap();
    fd.id().unwrap();

    /* CURRENTLY FAILS */
    // fd.refresh(5).unwrap();
    //
    // let interval: u64 = 5;
    // let ptr: *const u64 = &interval;
    // fd.overflow_period(ptr).unwrap();
    //
    // fd.pause_output().unwrap();
    //
    // fd.resume_output().unwrap();
}
