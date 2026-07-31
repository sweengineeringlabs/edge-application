//! `ClockError` — errors produced when reading time from a [`Clock`](crate::Clock).

use thiserror::Error;

/// Error produced when a clock reading cannot be interpreted.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ClockError {
    /// The clock reported a time earlier than the Unix epoch, so the elapsed
    /// duration since the epoch is undefined.
    #[error("clock time is before the unix epoch")]
    BeforeEpoch,
}
