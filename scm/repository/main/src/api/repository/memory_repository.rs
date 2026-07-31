//! `MemoryRepository` — heap-backed repository for testing and prototyping.

use std::collections::HashMap;
use std::hash::Hash;

use parking_lot::RwLock;

/// An in-memory repository backed by a `HashMap` protected by a `RwLock`.
///
/// Suitable for tests and in-process prototyping. The `store` field is
/// `pub(crate)` so that `core/` implementations can access it directly
/// without exposing raw storage to consumers.
pub struct MemoryRepository<T, Id>
where
    Id: Hash + Eq + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    pub(crate) store: RwLock<HashMap<Id, T>>,
}
