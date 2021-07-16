type Result<T, E> = std::result::Result<T, E>;

/// Errors related to system calls.
#[derive(Debug)]
pub enum SysErr {
    ReadFail,
    IoFail,
    IoArg,
    IoId,
}

/// Errors related to handling specific events.
#[derive(Debug)]
pub enum EventErr {
    InvalidEvent,
}
