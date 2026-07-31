//! [`LogDrainBuildResponse`] — wrapper for a constructed `LogDrain`.
// @allow: dto_types_must_serialize — holds a live `Box<dyn LogDrain>` factory
// result, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::LogDrain;

/// Result of [`ObserveBootstrap::build_log_drain`](crate::api::ObserveBootstrap::build_log_drain).
pub struct LogDrainBuildResponse {
    /// The constructed log drain.
    pub log_drain: Box<dyn LogDrain>,
}
