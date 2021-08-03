//! Errors for `ruperf stat`.
use thiserror::Error;

/// Parse errors for CLI
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Event")]
    InvalidEvent,
}
