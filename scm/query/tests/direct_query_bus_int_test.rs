//! Integration tests for `DirectQueryBus` — the zero-size in-process query bus marker.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_query::{DirectQueryBus, Query, QueryBus, QueryDispatchRequest, QueryError, QueryExecuteRequest, QueryResultResponse};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct Ok_(String);
impl Query for Ok_ {
    type Result = String;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

struct Err_;
impl Query for Err_ {
    type Result = String;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("gone".into())) })
    }
}

/// @covers: DirectQueryBus — is a zero-sized type (PhantomData<fn() -> R> is zero-sized)
#[test]
fn test_direct_query_bus_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DirectQueryBus<String>>(), 0);
}

/// @covers: DirectQueryBus — dispatches failing query
#[test]
fn test_direct_query_bus_dispatch_error_query_returns_err_error() {
    let bus = DirectQueryBus::<String>::new();
    let result = block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Err_) }));
    assert!(result.is_err());
}

/// @covers: DirectQueryBus — usable as `&dyn QueryBus`
#[test]
fn test_direct_query_bus_dyn_dispatch_returns_ok_edge() {
    let bus: &dyn QueryBus<Result = String> = &DirectQueryBus::<String>::new();
    let result = block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Ok_("hi".into())) }));
    assert_eq!(result.unwrap().result, "hi");
}
