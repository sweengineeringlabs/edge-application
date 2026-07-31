//! [`QueryResultResponse`] — wrapper for a query's result value.

/// Result of [`Query::execute`](crate::api::query::traits::Query::execute) or
/// [`QueryBus::dispatch`](crate::api::query::traits::QueryBus::dispatch).
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueryResultResponse<R> {
    /// The value produced by the query.
    pub result: R,
}
