//! [`ProviderBuildResponse`] — wrapper for a wired `Bootstrap`.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Bootstrap>` factory
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::Bootstrap;

/// Result of [`AppServiceProvider::build`](crate::api::AppServiceProvider::build).
pub struct ProviderBuildResponse {
    /// The configured bootstrap produced from the wired service graph.
    pub bootstrap: Box<dyn Bootstrap>,
}
