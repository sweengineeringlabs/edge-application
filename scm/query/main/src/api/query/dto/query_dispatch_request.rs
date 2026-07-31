//! [`QueryDispatchRequest`] — request to dispatch a query through a bus.
// @allow: dto_types_must_serialize — holds a live `Box<dyn Query<Result = R>>` to
// dispatch, not wire-format data; a trait object cannot derive Serialize/Deserialize.

use crate::api::query::traits::Query;

/// Request to dispatch `query` through a [`QueryBus`](crate::api::query::traits::QueryBus).
pub struct QueryDispatchRequest<R> {
    /// The query to dispatch.
    pub query: Box<dyn Query<Result = R>>,
}
