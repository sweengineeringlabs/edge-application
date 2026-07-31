//! Layer-level coverage for `api/query/dto/*.rs` request/response types.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_query::{
    Query, QueryDispatchRequest, QueryError, QueryExecuteRequest, QueryNameRequest,
    QueryNameResponse, QueryResultResponse,
};
use futures::future::BoxFuture;

struct Echo(String);
impl Query for Echo {
    type Result = String;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

/// @covers: QueryExecuteRequest
#[test]
fn test_query_execute_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<QueryExecuteRequest>(), 0);
    let _ = QueryExecuteRequest;
}

/// @covers: QueryNameRequest
#[test]
fn test_query_name_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<QueryNameRequest>(), 0);
    let _ = QueryNameRequest;
}

/// @covers: QueryNameResponse
#[test]
fn test_query_name_response_holds_name_happy() {
    let r = QueryNameResponse { name: "get-order" };
    assert_eq!(r.name, "get-order");
}

/// @covers: QueryResultResponse
#[test]
fn test_query_result_response_holds_result_happy() {
    let r = QueryResultResponse { result: 42u64 };
    assert_eq!(r.result, 42);
}

/// @covers: QueryDispatchRequest
#[test]
fn test_query_dispatch_request_holds_boxed_query_happy() {
    let req = QueryDispatchRequest { query: Box::new(Echo("hi".into())) };
    let result = futures::executor::block_on(req.query.execute(QueryExecuteRequest)).unwrap();
    assert_eq!(result.result, "hi");
}
