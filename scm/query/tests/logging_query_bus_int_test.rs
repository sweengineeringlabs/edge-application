//! Integration tests for `LoggingQueryBus<R>` — delegates dispatch and logs outcome.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use edge_application_query::{
    DirectQueryBus, LoggingQueryBus, NoopQueryBus, Query, QueryBus, QueryDispatchRequest,
    QueryError, QueryExecuteRequest, QueryResultResponse,
};
use futures::executor::block_on;
use futures::future::BoxFuture;

fn noop_inner() -> Arc<dyn QueryBus<Result = String>> {
    Arc::new(NoopQueryBus::<String>::new())
}

fn direct_inner() -> Arc<dyn QueryBus<Result = String>> {
    Arc::new(DirectQueryBus::<String>::new())
}

struct OkQuery(String);
impl Query for OkQuery {
    type Result = String;
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

struct ErrQuery;
impl Query for ErrQuery {
    type Result = String;
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("gone".into())) })
    }
}

/// @covers: LoggingQueryBus — constructed from noop inner via struct literal
#[test]
fn test_logging_query_bus_new_from_noop_inner_happy() {
    let bus = LoggingQueryBus::<String> { inner: noop_inner() };
    // Verify that the inner Arc was properly stored by checking we can dispatch through it
    let result = block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(ErrQuery) }));
    assert!(result.is_err(), "noop inner should return NotFound error");
}

/// @covers: LoggingQueryBus — dispatch ok query returns value
#[tokio::test]
async fn test_logging_query_bus_dispatch_ok_query_returns_value_happy() {
    let bus = LoggingQueryBus::<String> { inner: direct_inner() };
    let result = bus.dispatch(QueryDispatchRequest { query: Box::new(OkQuery("pong".into())) }).await;
    assert_eq!(result.unwrap().result, "pong");
}

/// @covers: LoggingQueryBus — dispatch error query propagates err
#[tokio::test]
async fn test_logging_query_bus_dispatch_error_query_returns_err_error() {
    let bus = LoggingQueryBus::<String> { inner: noop_inner() };
    assert!(bus.dispatch(QueryDispatchRequest { query: Box::new(ErrQuery) }).await.is_err());
}

/// @covers: LoggingQueryBus — usable as `dyn QueryBus`
#[test]
fn test_logging_query_bus_dyn_dispatch_edge() {
    let bus: Arc<dyn QueryBus<Result = String>> =
        Arc::new(LoggingQueryBus::<String> { inner: direct_inner() });
    let result = block_on(bus.dispatch(QueryDispatchRequest { query: Box::new(OkQuery("hi".into())) }));
    assert_eq!(result.unwrap().result, "hi");
}
