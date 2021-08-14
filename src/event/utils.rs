//! SPDX-License-Identifier: GPL-2.0
//! This file may contain more items in the
//! future. For now it defines a generic `Result`
//! type for handling system call failures and
//! invalid event requests.

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
