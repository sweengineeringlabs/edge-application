//! [`RegisterRequest`] — request to register an entry under an id.
// @allow: dto_types_must_serialize — holds a live `Arc<V>` where `V: ?Sized` is
// typically a trait object, not wire-format data; an unsized type param cannot
// derive Serialize/Deserialize.

use std::sync::Arc;

/// Request to register `entry` under `id`, replacing any existing entry.
pub struct RegisterRequest<V: ?Sized + Send + Sync> {
    /// The id to register the entry under.
    pub id: String,
    /// The entry to store.
    pub entry: Arc<V>,
}
