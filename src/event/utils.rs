/// Result type for anytime
/// `ioctl()` returns -1.
// TODO: get value of `errno`
// for more accurate error handling.
type Result<T> = std::result::Result<T, IoError>;

/// This error type should
/// eventually be used to
/// provide information
/// based on value of `errno`.
#[derive(Debug)]
pub enum IoError {
    SysCallFail,
    InvalidArg,
    InvalidId,
}
