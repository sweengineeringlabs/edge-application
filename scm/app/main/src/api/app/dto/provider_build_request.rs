//! [`ProviderBuildRequest`] — zero-sized marker for requesting a wired `Bootstrap`.

/// Request to build a configured [`Bootstrap`](crate::api::Bootstrap) from the wired service graph.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProviderBuildRequest;
