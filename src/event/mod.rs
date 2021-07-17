//! For notes about Rust bindings necessary for
//! `perf_event_open()` see /src/bindings/perf_event.rs.

//! Disable cargo build warnings created due to using bindgen
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod event;
mod fd;
mod sys;
mod utils;

pub fn perf_event_hello() {
    println!("hello from your friendly perf_event file");
}
