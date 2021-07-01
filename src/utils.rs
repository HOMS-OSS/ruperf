//! Utility functions
use thiserror::Error;

/// Parse errors for CLI
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Invalid Event")]
    InvalidEvent,
}
