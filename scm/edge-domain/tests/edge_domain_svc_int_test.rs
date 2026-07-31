//! Integration tests for saf factory functions.
#![cfg(all(feature = "repository", feature = "query", feature = "command", feature = "event"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::DomainRuntime;
use edge_application::{
    Command, CommandBus, CommandError, Domain, QueryBus, QueryError, QueryableRepository,
    Repository, RepositoryIdRequest, RepositorySaveRequest, SpecRequest,
};
use edge_application::{DirectCommandBusRequest, NoopEventPublisherRequest};
use edge_application_command::{CommandDispatchRequest, ExecutionRequest, NameRequest, NameResponse};
use edge_application_handler::{
    EmptinessRequest as HandlerEmptinessRequest, LenRequest as HandlerLenRequest,
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

/// @covers: new_handler_registry
#[test]
fn test_factory_fn_new_handler_registry_returns_empty_arc_registry() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 0);
}

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository_returns_arc_repository() {
    let _: Arc<dyn Repository<Entity = String, Id = u32>> = Domain.new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository_returns_arc_queryable_repository() {
    let _: Arc<dyn QueryableRepository<Entity = String, Id = u32>> =
        Domain.new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_saves_and_finds_entity() {
    let repo: Arc<dyn Repository<Entity = String, Id = u32>> = Domain.new_in_memory_repository();
    repo.save(RepositorySaveRequest {
        id: 1u32,
        entity: "hello".to_string(),
    })
    .await
    .unwrap();
    let found = repo.find(RepositoryIdRequest { id: &1u32 }).await.unwrap();
    assert_eq!(found.entity.as_deref(), Some("hello"));
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_finds_by_spec() {
    use edge_application::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};
    struct LongStr;
    impl Spec for LongStr {
        type Entity = String;

        fn matches(
            &self,
            req: SpecMatchesRequest<'_, String>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse {
                matches: req.entity.len() > 3,
            })
        }
    }
    let repo: Arc<dyn QueryableRepository<Entity = String, Id = u32>> =
        Domain.new_in_memory_queryable_repository();
    repo.save(RepositorySaveRequest {
        id: 1u32,
        entity: "hi".to_string(),
    })
    .await
    .unwrap();
    repo.save(RepositorySaveRequest {
        id: 2u32,
        entity: "hello".to_string(),
    })
    .await
    .unwrap();
    let results = repo
        .find_by(SpecRequest {
            spec: Box::new(LongStr),
        })
        .await
        .unwrap()
        .items;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "hello");
}

/// @covers: direct_command_bus
#[tokio::test]
async fn test_factory_fn_direct_command_bus_dispatches_command_inline() {
    use futures::future::BoxFuture;
    struct PingCommand;
    impl Command for PingCommand {
        fn name(&self, _req: NameRequest) -> Result<NameResponse, CommandError> {
            Ok(NameResponse {
                name: "ping".to_string(),
            })
        }
        fn execute(&self, _req: ExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }
    let bus: Arc<dyn CommandBus> = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    assert!(bus
        .dispatch(CommandDispatchRequest {
            command: Box::new(PingCommand)
        })
        .await
        .is_ok());
}

/// @covers: noop_event_publisher
#[tokio::test]
async fn test_factory_fn_noop_event_publisher_silently_discards_events() {
    use edge_application::{
        DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError,
        EventOccurredAtRequest, EventOccurredAtResponse, EventPublisherPublishRequest,
        EventTypeRequest, EventTypeResponse,
    };
    use std::time::SystemTime;
    struct AnyEvent;
    impl DomainEvent for AnyEvent {
        fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
            Ok(EventTypeResponse { event_type: "any" })
        }
        fn aggregate_id(
            &self,
            _req: EventAggregateIdRequest,
        ) -> Result<EventAggregateIdResponse<'_>, EventError> {
            Ok(EventAggregateIdResponse {
                aggregate_id: "id-1",
            })
        }
        fn occurred_at(
            &self,
            _req: EventOccurredAtRequest,
        ) -> Result<EventOccurredAtResponse, EventError> {
            Ok(EventOccurredAtResponse {
                occurred_at: SystemTime::now(),
            })
        }
    }
    let publisher = Domain
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    assert!(publisher
        .publish(EventPublisherPublishRequest { event: &AnyEvent })
        .await
        .is_ok());
}

/// @covers: direct_query_bus
#[tokio::test]
async fn test_factory_fn_direct_query_bus_dispatches_query_inline() {
    use edge_application::{
        Query, QueryDispatchRequest, QueryExecuteRequest, QueryNameRequest, QueryNameResponse,
        QueryResultResponse,
    };
    use futures::future::BoxFuture;
    struct EchoQuery(String);
    impl Query for EchoQuery {
        type Result = String;
        fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
            Ok(QueryNameResponse { name: "echo" })
        }
        fn execute(
            &self,
            _req: QueryExecuteRequest,
        ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
            let v = self.0.clone();
            Box::pin(async move { Ok(QueryResultResponse { result: v }) })
        }
    }
    let bus: Arc<dyn QueryBus<Result = String>> = Domain.direct_query_bus();
    let result = bus
        .dispatch(QueryDispatchRequest {
            query: Box::new(EchoQuery("pong".into())),
        })
        .await
        .unwrap();
    assert_eq!(result.result, "pong");
}
