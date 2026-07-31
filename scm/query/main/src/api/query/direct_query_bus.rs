//! API-layer type for the direct (in-process) query bus.

use std::marker::PhantomData;

/// Marker type describing a `QueryBus` that dispatches queries inline,
/// calling `query.execute()` directly in the same task with no queuing.
///
/// The type parameter `R` is the result type returned by dispatched queries.
/// The concrete implementation lives in `core::query::direct_query_bus`.
pub struct DirectQueryBus<R>(pub(crate) PhantomData<fn() -> R>);
