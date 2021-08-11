//! An `Event` abstracts away
//! initializing `perf_event_attr`
//! structs for arbitrary events;
//! and the need to use `FileDesc` methods
//! for interacting with `perf_event`
//! related file descriptors.

use crate::bindings::*;
use crate::event::fd;
use crate::event::utils::*;
use crate::stat::StatEvent;

///Event enum contains file descriptor and event type
//simple starting options. Add more as needed
pub struct Event {
    pub fd: fd::FileDesc,
    pub event: StatEvent,
}

/// Initialize perf attributes. Currently set up to match on intended event.
/// Returns the initialized perf_event_attr data structure or an error.
pub fn event_open(event: &StatEvent) -> Result<perf_event_attr, EventErr> {
    match &event {
        StatEvent::Cycles => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        StatEvent::Instructions => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        StatEvent::TaskClock => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_SOFTWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_sw_ids_PERF_COUNT_SW_TASK_CLOCK as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        StatEvent::ContextSwitches => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_SOFTWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_sw_ids_PERF_COUNT_SW_CONTEXT_SWITCHES as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(0);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        _ => Err(EventErr::InvalidEvent),
    }
}

impl Event {
    /// Construct a new event.
    pub fn new(event: StatEvent, pid: Option<i32>) -> Self {
        let e: &mut perf_event_attr = &mut event_open(&event).unwrap();
        let fd = fd::FileDesc::new(e, pid, -1, -1);
        Self { fd, event }
    }
    /// Start the counter on an event.
    pub fn start_counter(&self) -> Result<isize, SysErr> {
        match self.fd.enable() {
            Ok(_) => self.fd.read(),
            Err(e) => Err(e),
        }
    }
    ///Stop the counter on an event.
    pub fn stop_counter(&self) -> Result<isize, SysErr> {
        match self.fd.disable() {
            Ok(_) => self.fd.read(),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[test]
fn cycles_open_test() {
    let event = Event::new(StatEvent::Cycles, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
fn inst_open_test() {
    let event = Event::new(StatEvent::Instructions, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
fn taskclock_open_test() {
    let event = Event::new(StatEvent::TaskClock, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
fn cs_open_test() {
    let event = Event::new(StatEvent::ContextSwitches, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt_2, -1);
}
