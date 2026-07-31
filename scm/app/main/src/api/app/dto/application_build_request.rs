//! [`ApplicationBuildRequest`] — zero-sized marker for requesting a ready-to-run `Application`.

/// Request to build and return a ready-to-run [`Application`](crate::api::Application).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ApplicationBuildRequest;
