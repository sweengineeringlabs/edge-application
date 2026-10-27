//! Integration tests for `Command`, `Query`, `CommandBus`, and `QueryBus`.
#![cfg(all(feature = "query", feature = "command"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{
    Command, CommandBus, CommandError, Query, QueryBus, QueryDispatchRequest, QueryError,
    QueryExecuteRequest, QueryNameRequest, QueryNameResponse, QueryResultResponse,
};
use edge_application_command::{
    CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse,
};
use futures::future::BoxFuture;
use std::sync::Arc;

// ── Command fixtures ─────────────────────────────────────────────────────────

struct NoopCommand;

impl Command for NoopCommand {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "noop".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct FailingCommand;

impl Command for FailingCommand {
    fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
        Ok(NameResponse {
            name: "failing".to_string(),
        })
    }
    fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

// ── Query fixtures ───────────────────────────────────────────────────────────

struct EchoQuery {
    value: String,
}

impl Query for EchoQuery {
    type Result = String;
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "echo" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.value.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

struct MissingQuery;

impl Query for MissingQuery {
    type Result = String;
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "missing" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("resource-42".into())) })
    }
}

// ── CommandBus fixture ───────────────────────────────────────────────────────

struct DirectCommandBus;

impl CommandBus for DirectCommandBus {
    fn dispatch(&self, req: CommandDispatchRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async move { req.command.execute(ExecutionRequest).await })
    }
}

// ── QueryBus fixture ─────────────────────────────────────────────────────────

struct DirectQueryBus;

impl QueryBus for DirectQueryBus {
    type Result = String;
    fn dispatch(
        &self,
        req: QueryDispatchRequest<String>,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async move { req.query.execute(QueryExecuteRequest).await })
    }
}

// ── tests ────────────────────────────────────────────────────────────────────

/// @covers: Command::execute
#[tokio::test]
async fn test_command_trait_execute_returns_ok_on_success() {
    assert!(NoopCommand.execute(ExecutionRequest).await.is_ok());
}

/// @covers: Command::execute
#[tokio::test]
async fn test_command_trait_execute_returns_err_on_failure() {
    assert!(FailingCommand.execute(ExecutionRequest).await.is_err());
}

/// @covers: Command::name
#[test]
fn test_command_trait_name_returns_stable_identifier() {
    assert_eq!(NoopCommand.name(NameRequest).unwrap().name, "noop");
}

/// @covers: Query::execute
#[tokio::test]
async fn test_query_trait_execute_returns_result_without_mutation() {
    let q = EchoQuery {
        value: "pong".into(),
    };
    assert_eq!(q.execute(QueryExecuteRequest).await.unwrap().result, "pong");
}

/// @covers: Query::execute
#[tokio::test]
async fn test_query_trait_execute_returns_not_found_error() {
    let err = MissingQuery.execute(QueryExecuteRequest).await.unwrap_err();
    assert!(matches!(err, QueryError::NotFound(_)));
    assert!(err.to_string().contains("resource-42"));
}

/// @covers: CommandBus::dispatch
#[tokio::test]
async fn test_command_bus_trait_dispatch_delegates_to_command_execute() {
    let bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(NoopCommand)
        })
        .await
        .is_ok());
}

/// @covers: CommandBus::dispatch
#[tokio::test]
async fn test_command_bus_trait_dispatch_propagates_command_error() {
    let bus: Arc<dyn CommandBus> = Arc::new(DirectCommandBus);
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(FailingCommand)
        })
        .await
        .is_err());
}

/// @covers: QueryBus::dispatch
#[tokio::test]
async fn test_query_bus_trait_dispatch_returns_query_result() {
    let bus: Arc<dyn QueryBus<Result = String>> = Arc::new(DirectQueryBus);
    let result = bus
        .dispatch(QueryDispatchRequest {
            query: Box::new(EchoQuery {
                value: "hello".into(),
            }),
        })
        .await
        .unwrap();
    assert_eq!(result.result, "hello");
}

/// @covers: QueryBus::dispatch
#[tokio::test]
async fn test_query_bus_trait_dispatch_propagates_query_error() {
    let bus: Arc<dyn QueryBus<Result = String>> = Arc::new(DirectQueryBus);
    let err = bus
        .dispatch(QueryDispatchRequest {
            query: Box::new(MissingQuery),
        })
        .await
        .unwrap_err();
    assert!(matches!(err, QueryError::NotFound(_)));
}
