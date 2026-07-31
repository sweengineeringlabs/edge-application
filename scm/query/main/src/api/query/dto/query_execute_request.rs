//! [`QueryExecuteRequest`] — zero-sized marker for executing a query.

/// Request to execute a [`Query`](crate::api::query::traits::Query).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QueryExecuteRequest;
