//! `MemoryRegistry` — in-process [`Registry`](crate::Registry) backed by a `RwLock<HashMap>`.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// In-process [`Registry`](crate::Registry) implementation storing shared
/// entries keyed by string id. Concurrent reads and writes are guarded by a
/// `RwLock`.
pub struct MemoryRegistry<V: ?Sized + Send + Sync> {
    pub(crate) entries: RwLock<HashMap<String, Arc<V>>>,
}
