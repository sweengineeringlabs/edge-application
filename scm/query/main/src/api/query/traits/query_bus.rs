//! `QueryBus` trait — dispatches queries and returns their results.

use std::future::Future;
use std::pin::Pin;

use crate::api::query::QueryError;
use crate::api::query::dto::{QueryDispatchRequest, QueryResultResponse};

/// Dispatches [`Query`](super::Query) instances and returns their results.
pub trait QueryBus: Send + Sync {
    /// The result type produced by queries dispatched through this bus.
    type Result: Send + 'static;

    /// Dispatch a query and return its result.
    fn dispatch(
        &self,
        req: QueryDispatchRequest<Self::Result>,
    ) -> Pin<Box<dyn Future<Output = Result<QueryResultResponse<Self::Result>, QueryError>> + Send + '_>>;
}
