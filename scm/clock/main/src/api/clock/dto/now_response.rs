//! [`NowResponse`] — wrapper for the current instant.

use std::time::SystemTime;

/// Result of [`Clock::now`](crate::api::Clock::now).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NowResponse {
    /// The current instant.
    pub instant: SystemTime,
}
