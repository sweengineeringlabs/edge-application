//! [`RegistryLookupResponse`] — wrapper for a registry lookup result.
// @allow: dto_types_must_serialize — holds a live `Option<Arc<V>>` where
// `V: ?Sized` is typically a trait object, not wire-format data; an unsized
// type param cannot derive Serialize/Deserialize.

use std::sync::Arc;

/// Result of [`Registry::get`](crate::api::Registry::get).
pub struct RegistryLookupResponse<V: ?Sized + Send + Sync> {
    /// The resolved entry, or `None` if no entry is registered under the requested id.
    pub entry: Option<Arc<V>>,
}
