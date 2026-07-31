//! `Query` and `QueryBus` impls for noop types — NoopQuery always returns `Ok(())`;
//! NoopQueryBus always returns `Err(QueryError::NotFound)`.

use std::marker::PhantomData;

use futures::future::BoxFuture;

use crate::api::Query;
use crate::api::QueryBus;
use crate::api::NoopQuery;
use crate::api::NoopQueryBus;
use crate::api::QueryError;
use crate::api::{QueryDispatchRequest, QueryExecuteRequest, QueryResultResponse};

impl<R> NoopQueryBus<R> {
    /// Construct a new `NoopQueryBus`.
    pub fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for NoopQueryBus<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl Query for NoopQuery {
    type Result = ();

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<()>, QueryError>> {
        Box::pin(async { Ok(QueryResultResponse { result: () }) })
    }
}

impl<R: Send + 'static> QueryBus for NoopQueryBus<R> {
    type Result = R;

    fn dispatch(
        &self,
        _req: QueryDispatchRequest<R>,
    ) -> BoxFuture<'_, Result<QueryResultResponse<R>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("noop".into())) })
    }
}
