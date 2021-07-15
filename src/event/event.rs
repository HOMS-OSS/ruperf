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
pub fn event_open(event: &StatEvent) -> Result<fd::perf_event_attr, IoError> {
    match &event {
        StatEvent::Cycles => {
            let event_open = &mut fd::perf_event_attr {
                type_: fd::perf_type_id_PERF_TYPE_HARDWARE,
                size: std::mem::size_of::<fd::perf_event_attr>() as u32,
                config: fd::perf_hw_id_PERF_COUNT_HW_CPU_CYCLES as u64,
                ..Default::default()
            };
            event_open.set_disabled(1);
            event_open.set_exclude_kernel(1);
            event_open.set_exclude_hv(1);
            Ok(*event_open)
        }
        _ => return Err(IoError::InvalidArg),
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
    pub fn start_counter(&self) -> Result<isize, IoError> {
        self.fd.enable().unwrap();
        self.fd.read()
    }

    ///Stop the counter on an event
    pub fn stop_counter(&self) -> Result<isize, IoError> {
        self.fd.disable().unwrap();
        self.fd.read()
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
