//! [`ApplicationRunResponse`] — wrapper for a terminated application run.

/// Result of [`Application::run`](crate::api::Application::run) terminating cleanly.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationRunResponse;
