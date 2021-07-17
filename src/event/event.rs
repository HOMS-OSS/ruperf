use crate::event::fd;
use crate::event::sys::sys;
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
pub fn event_open(event: &StatEvent) -> Result<fd::perf_event_attr, EventErr> {
    match &event {
        StatEvent::Cycles => {
            let event_open = &mut fd::perf_event_attr {
                type_: sys::perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<fd::perf_event_attr>() as u32,
                config: sys::perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        _ => return Err(EventErr::InvalidEvent),
    }
}
impl Event {
    /// Construct a new event
    pub fn new(event: StatEvent) -> Self {
        let e: &mut fd::perf_event_attr = &mut event_open(&event).unwrap();
        let fd = fd::FileDesc::new(e, 0, -1, -1);
        Self {
            fd: fd,
            event: event,
        }
    }

    /// Start the counter on an event
    pub fn start_counter(&self) -> Result<isize, SysErr> {
        match self.fd.enable() {
            Ok(_) => self.fd.read(),
            Err(e) => Err(e),
        }
    }

    ///Stop the counter on an event
    pub fn stop_counter(&self) -> Result<isize, SysErr> {
        match self.fd.disable() {
            Ok(_) => self.fd.read(),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
#[test]
fn event_open_test() {
    let event = Event::new(StatEvent::Cycles);
    let cnt: isize = event.start_counter().unwrap();
    assert_ne!(cnt, 0);
    assert_ne!(cnt, -1);
    let cnt_2 = event.stop_counter().unwrap();
    assert_ne!(cnt, cnt_2);
    assert!(cnt < cnt_2);
}
