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

const PERF_EVENT_ATTR_SIZE: u32 = std::mem::size_of::<perf_event_attr>() as u32;

///Event enum contains file descriptor and event type
//simple starting options. Add more as needed
pub struct Event {
    pub fd: fd::FileDesc,
    pub event: StatEvent,
}

/// Initialize perf attributes. Currently set up to match on intended event.
/// Returns the initialized perf_event_attr data structure or an error.
pub fn event_open(event: &StatEvent) -> Result<perf_event_attr, EventErr> {
    // To measure hardware CPU cache events
    // when `type_` is set to `PERF_TYPE_HW_CACHE`
    // the value of config must be computed.
    // See the `perf_event_open()` man page for details.
    let cache_config = |id, op, rs| (id as u64) | ((op as u64) << 8) | ((rs as u64) << 16);
    match &event {
        StatEvent::Cycles => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HARDWARE,
                size: PERF_EVENT_ATTR_SIZE,
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
                size: PERF_EVENT_ATTR_SIZE,
                config: perf_hw_id_PERF_COUNT_HW_INSTRUCTIONS as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
<<<<<<< HEAD
        StatEvent::TaskClock => {
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_SOFTWARE,
                size: std::mem::size_of::<perf_event_attr>() as u32,
                config: perf_sw_ids_PERF_COUNT_SW_TASK_CLOCK as u64,
=======
        StatEvent::L1DCacheRead => {
            let config: u64 = cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1D,
                perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_READ,
                perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            );
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HW_CACHE,
                size: PERF_EVENT_ATTR_SIZE,
                config,
>>>>>>> Feat: added cache events
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
<<<<<<< HEAD
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
=======
        StatEvent::L1DCacheWrite => {
            let config: u64 = cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1D,
                perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_WRITE,
                perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_ACCESS,
            );
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HW_CACHE,
                size: PERF_EVENT_ATTR_SIZE,
                config,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        StatEvent::L1DCacheReadMiss => {
            let config: u64 = cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1D,
                perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_READ,
                perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_MISS,
            );
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HW_CACHE,
                size: PERF_EVENT_ATTR_SIZE,
                config,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        StatEvent::L1ICacheReadMiss => {
            let config: u64 = cache_config(
                perf_hw_cache_id_PERF_COUNT_HW_CACHE_L1I,
                perf_hw_cache_op_id_PERF_COUNT_HW_CACHE_OP_READ,
                perf_hw_cache_op_result_id_PERF_COUNT_HW_CACHE_RESULT_MISS,
            );
            let event_open = &mut perf_event_attr {
                type_: perf_type_id_PERF_TYPE_HW_CACHE,
                size: PERF_EVENT_ATTR_SIZE,
                config,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        _ => Err(EventErr::InvalidEvent)
>>>>>>> Feat: added cache events
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

    /// Reset the counter to 0.
    pub fn reset_counter(&self) -> Result<(), SysErr> {
        match self.fd.reset() {
            Ok(_) => Ok(()),
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
<<<<<<< HEAD
fn taskclock_open_test() {
    let event = Event::new(StatEvent::TaskClock, None);
=======
fn l1_data_cache_read_open_test() {
    let event = Event::new(StatEvent::L1DCacheRead, None);
>>>>>>> Feat: added cache events
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
<<<<<<< HEAD
fn cs_open_test() {
    let event = Event::new(StatEvent::ContextSwitches, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt_2, -1);
=======
fn l1_data_cache_write_open_test() {
    let event = Event::new(StatEvent::L1DCacheWrite, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
fn l1_data_cache_read_miss_open_test() {
    let event = Event::new(StatEvent::L1DCacheReadMiss, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}

#[test]
fn l1_inst_cache_read_miss_open_test() {
    let event = Event::new(StatEvent::L1ICacheReadMiss, None);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
>>>>>>> Feat: added cache events
}
