//! `QueryBus` trait — dispatches queries and returns their results.

use crate::api::query::dto::QueryDispatchRequest;
use crate::api::query::traits::query::QueryFuture;

/// Dispatches [`Query`](super::Query) instances and returns their results.
pub trait QueryBus: Send + Sync {
    /// The result type produced by queries dispatched through this bus.
    type Result: Send + 'static;

    /// Dispatch a query and return its result.
    fn dispatch(&self, req: QueryDispatchRequest<Self::Result>) -> QueryFuture<'_, Self::Result>;
}
