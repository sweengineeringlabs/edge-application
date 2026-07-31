//! [`DrainResponse`] — wrapper for the active `LogDrain`.
// @allow: dto_types_must_serialize — holds a `&dyn LogDrain` reference, not
// wire-format data; a trait object reference cannot derive Serialize/Deserialize.

use crate::api::context::observe::LogDrain;

/// Result of [`ObserverContext::drain`](crate::api::context::observe::ObserverContext::drain).
pub struct DrainResponse<'a> {
    /// The active log drain.
    pub drain: &'a dyn LogDrain,
}
