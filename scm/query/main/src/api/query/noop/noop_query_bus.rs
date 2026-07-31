//! API-layer marker type for the noop query bus.

use std::marker::PhantomData;

/// A [`QueryBus`](crate::QueryBus) that always returns
/// `Err(`[`QueryError::NotFound`](crate::QueryError)`)`.
///
/// Use for testing and bring-up where a real bus is not wired yet.
/// The concrete `QueryBus` implementation lives in `core::query::noop_query`.
pub struct NoopQueryBus<R>(pub(crate) PhantomData<fn() -> R>);
