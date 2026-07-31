//! [`ApplicationBuildResponse`] — wrapper for a ready-to-run `Application`.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Application>` factory
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::Application;

/// Result of [`Bootstrap::build`](crate::api::Bootstrap::build).
pub struct ApplicationBuildResponse {
    /// The ready-to-run application.
    pub application: Box<dyn Application>,
}
