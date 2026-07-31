//! `NoopQuery` — a no-op `Query` implementation that always returns `Ok(())`.

/// A no-op [`Query`](crate::api::query::traits::Query) that always succeeds with a `()` result.
///
/// Useful as a placeholder or stand-in where a concrete `Query` implementor
/// is required but meaningful query semantics are not needed (e.g. wiring,
/// feature-gated code paths, or structural tests).
pub struct NoopQuery;
