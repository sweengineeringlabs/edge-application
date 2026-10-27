//! `Query` trait — a read operation that never mutates domain state.

use std::future::Future;
use std::pin::Pin;

use crate::api::query::dto::{
    QueryExecuteRequest, QueryNameRequest, QueryNameResponse, QueryResultResponse,
};
use crate::api::query::QueryError;

/// Boxed future returned by query execution.
pub type QueryFuture<'a, T> =
    Pin<Box<dyn Future<Output = Result<QueryResultResponse<T>, QueryError>> + Send + 'a>>;

/// A named read operation that returns data without mutating state.
pub trait Query: Send + Sync {
    /// The type returned on success.
    type Result: Send + 'static;

    /// Stable name identifying this query type.
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "query" })
    }

    /// Execute the query and return the result.
    fn execute(&self, req: QueryExecuteRequest) -> QueryFuture<'_, Self::Result>;
}
