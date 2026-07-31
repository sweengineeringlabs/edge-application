//! [`ElapsedSinceEpochResponse`] — wrapper for the elapsed duration since the Unix epoch.

use std::time::Duration;

/// Result of [`Clock::elapsed_since_epoch`](crate::api::Clock::elapsed_since_epoch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ElapsedSinceEpochResponse {
    /// The elapsed duration since the Unix epoch.
    pub duration: Duration,
}
