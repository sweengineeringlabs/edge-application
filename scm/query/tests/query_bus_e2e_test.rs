//! SAF facade tests — `QueryBus` trait via `DirectQueryBus`.
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
        Box::pin(async { Err(QueryError::Internal("boom".into())) })
    }
}

/// @covers: QueryBus::dispatch — success
#[test]
fn test_dispatch_ok_query_returns_result_happy() {
    let bus: DirectQueryBus<String> = DirectQueryBus::new();
    let result = block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Ok_("pong".into())) }));
    assert_eq!(result.unwrap().result, "pong");
}

/// @covers: QueryBus::dispatch — failure propagates
#[test]
fn test_dispatch_failing_query_returns_err_error() {
    let bus: DirectQueryBus<String> = DirectQueryBus::new();
    assert!(block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Err_) })).is_err());
}

/// @covers: QueryBus::dispatch — multiple dispatches independent
#[test]
fn test_dispatch_multiple_sequential_queries_are_independent_edge() {
    let bus: DirectQueryBus<String> = DirectQueryBus::new();
    assert!(block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Ok_("a".into())) })).is_ok());
    assert!(block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Err_) })).is_err());
    assert!(block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(Ok_("b".into())) })).is_ok());
}
