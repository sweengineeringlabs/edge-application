//! Integration tests for `NoopQuery` — the no-op `Query` implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_query::{NoopQuery, Query, QueryExecuteRequest, QueryNameRequest};
use futures::executor::block_on;

/// @covers: NoopQuery::execute — always returns Ok(())
#[test]
fn test_execute_noop_query_returns_ok_happy() {
    let result = block_on(NoopQuery.execute(QueryExecuteRequest));
    assert_eq!(result.unwrap().result, (), "noop query should return Ok(())");
}

/// @covers: NoopQuery::name — returns default "query"
#[test]
fn test_name_noop_query_returns_default_name_error() {
    assert_eq!(NoopQuery.name(QueryNameRequest).unwrap().name, "query");
}

/// @covers: NoopQuery::execute — repeated calls are independent
#[test]
fn test_execute_repeated_calls_are_independent_edge() {
    let result1 = block_on(NoopQuery.execute(QueryExecuteRequest));
    let result2 = block_on(NoopQuery.execute(QueryExecuteRequest));
    assert_eq!(result1.unwrap().result, (), "first noop query call should return Ok(())");
    assert_eq!(result2.unwrap().result, (), "second noop query call should return Ok(())");
}
