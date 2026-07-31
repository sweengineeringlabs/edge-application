//! SAF facade tests — `Query` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_query::{Query, QueryError, QueryExecuteRequest, QueryNameRequest, QueryNameResponse, QueryResultResponse};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct GetStr(String);
impl Query for GetStr {
    type Result = String;

    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: &self.0 })
    }

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

struct Fail;
impl Query for Fail {
    type Result = String;

    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("x".into())) })
    }
}

/// @covers: Query::name — configured name returned
#[test]
fn test_name_configured_value_returned_happy() {
    assert_eq!(GetStr("get-order".into()).name(QueryNameRequest).unwrap().name, "get-order");
}

/// @covers: Query::name — default impl returns "query"
#[test]
fn test_name_default_impl_returns_query_error() {
    assert_eq!(Fail.name(QueryNameRequest).unwrap().name, "query");
}

/// @covers: Query::name — via dyn dispatch
#[test]
fn test_name_via_dyn_dispatch_returns_name_edge() {
    let q: &dyn Query<Result = String> = &GetStr("x".into());
    assert_eq!(q.name(QueryNameRequest).unwrap().name, "x");
}

/// @covers: Query::execute — success
#[test]
fn test_execute_ok_query_returns_ok_happy() {
    assert_eq!(block_on(GetStr("hi".into()).execute(QueryExecuteRequest)).unwrap().result, "hi");
}

/// @covers: Query::execute — failure propagates
#[test]
fn test_execute_failing_query_returns_err_error() {
    assert!(block_on(Fail.execute(QueryExecuteRequest)).is_err());
}

/// @covers: Query::execute — repeated calls independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let q = GetStr("z".into());
    assert_eq!(block_on(q.execute(QueryExecuteRequest)).unwrap().result, "z", "first call should return correct value");
    assert_eq!(block_on(q.execute(QueryExecuteRequest)).unwrap().result, "z", "second call should return correct value");
}
