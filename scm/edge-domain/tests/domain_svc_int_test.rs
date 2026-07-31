//! Integration tests for saf/edge_application_svc public API.
//!
//! Covers all factory functions in edge_application_svc.rs
#![cfg(all(feature = "repository", feature = "validator", feature = "handler", feature = "command", feature = "event", feature = "query"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::*;
use edge_application_handler::{
    EmptinessRequest as HandlerEmptinessRequest, ExecutionRequest, HandlerContext,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

/// @covers: echo_handler
#[test]
fn test_echo_handler() {
    let _: Arc<dyn edge_application::Handler<Request = TextPayload, Response = TextPayload>> =
        Domain.echo_handler("id", "/path");
}

/// @covers: echo_handler
#[tokio::test]
async fn test_echo_handler_returns_input_as_output() {
    let h = Domain.echo_handler::<TextPayload>("echo", "/echo");
    let security = SecurityContext::unauthenticated();
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: bus.as_ref(),
        observer: observer.as_ref(),
    };
    assert_eq!(
        h.execute(ExecutionRequest {
            req: TextPayload("ping".to_string()),
            ctx: &ctx
        })
        .await
        .unwrap(),
        TextPayload("ping".to_string())
    );
}

/// @covers: new_handler_registry
#[test]
fn test_new_handler_registry_returns_empty_registry() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
}

/// @covers: new_in_memory_repository
#[test]
fn test_new_in_memory_repository() {
    let _: Arc<dyn edge_application::Repository<Entity = String, Id = u32>> =
        Domain.new_in_memory_repository();
}

/// @covers: new_in_memory_queryable_repository
#[test]
fn test_new_in_memory_queryable_repository() {
    let _: Arc<dyn edge_application::QueryableRepository<Entity = String, Id = u32>> =
        Domain.new_in_memory_queryable_repository();
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_returns_functional_store() {
    use edge_application::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};
    struct Any;
    impl Spec for Any {
        type Entity = String;

        fn matches(
            &self,
            _req: SpecMatchesRequest<'_, String>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse { matches: true })
        }
    }
    let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
    repo.save(RepositorySaveRequest {
        id: 1u32,
        entity: "x".to_string(),
    })
    .await
    .unwrap();
    assert_eq!(
        repo.count_by(SpecRequest {
            spec: Box::new(Any)
        })
        .await
        .unwrap()
        .count,
        1
    );
}

/// @covers: new_in_memory_repository
#[tokio::test]
async fn test_new_in_memory_repository_saves_and_finds_entity() {
    let repo: Arc<dyn edge_application::Repository<Entity = String, Id = u32>> =
        Domain.new_in_memory_repository::<String, u32>();
    repo.save(RepositorySaveRequest {
        id: 1u32,
        entity: "x".to_string(),
    })
    .await
    .unwrap();
    assert!(repo
        .find(RepositoryIdRequest { id: &1u32 })
        .await
        .unwrap()
        .entity
        .is_some());
}

/// @covers: new_in_memory_queryable_repository
#[tokio::test]
async fn test_new_in_memory_queryable_repository_supports_count_by() {
    use edge_application::{RepositoryError, Spec, SpecMatchesRequest, SpecMatchesResponse};
    struct Any;
    impl Spec for Any {
        type Entity = String;

        fn matches(
            &self,
            _req: SpecMatchesRequest<'_, String>,
        ) -> Result<SpecMatchesResponse, RepositoryError> {
            Ok(SpecMatchesResponse { matches: true })
        }
    }
    let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
    repo.save(RepositorySaveRequest {
        id: 1u32,
        entity: "x".to_string(),
    })
    .await
    .unwrap();
    assert_eq!(
        repo.count_by(SpecRequest {
            spec: Box::new(Any)
        })
        .await
        .unwrap()
        .count,
        1
    );
}

/// @covers: validate_config
#[test]
fn test_validate_config_returns_ok_for_valid_input() {
    use edge_application::{Validator, ValidatorError};
    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(
            &self,
            _req: edge_application_validator::ValidationRequest,
        ) -> Result<edge_application_validator::ValidationResponse, ValidatorError> {
            Ok(edge_application_validator::ValidationResponse)
        }
    }
    assert_eq!(Domain.validate_config(&AlwaysValid), Ok(()));
}

/// @covers: validate_config
#[test]
fn test_validate_config_returns_err_for_invalid_input() {
    use edge_application::{Validator, ValidatorError};
    struct AlwaysInvalid;
    impl Validator for AlwaysInvalid {
        fn validate(
            &self,
            _req: edge_application_validator::ValidationRequest,
        ) -> Result<edge_application_validator::ValidationResponse, ValidatorError> {
            Err(ValidatorError::Invalid("bad".into()))
        }
    }
    assert!(Domain.validate_config(&AlwaysInvalid).is_err());
}

/// @covers: direct_command_bus
#[test]
fn test_direct_command_bus_returns_arc_command_bus() {
    let bus = Domain
        .direct_command_bus(DirectCommandBusRequest)
        .unwrap()
        .bus;
    let _: Arc<dyn edge_application::CommandBus> = bus;
}

/// @covers: noop_event_publisher
#[test]
fn test_noop_event_publisher_returns_arc_event_publisher() {
    let pub_ = Domain
        .noop_event_publisher(NoopEventPublisherRequest)
        .unwrap()
        .publisher;
    let _: Arc<dyn edge_application::EventPublisher> = pub_;
}

/// @covers: direct_query_bus
#[test]
fn test_direct_query_bus_returns_arc_query_bus() {
    let bus = Domain.direct_query_bus::<String>();
    let _: Arc<dyn edge_application::QueryBus<Result = String>> = bus;
}

#[derive(Clone)]
struct AnyEvent;
impl edge_application::DomainEvent for AnyEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.any",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse { aggregate_id: "id" })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: std::time::SystemTime::now(),
        })
    }
}

/// @covers: new_in_memory_event_store
#[test]
fn test_new_in_memory_event_store_returns_arc_event_store() {
    let _: Arc<dyn edge_application::EventStore<Event = AnyEvent>> =
        Domain.new_in_memory_event_store::<AnyEvent>();
}

/// @covers: in_process_event_bus
#[test]
fn test_in_process_event_bus_factory_returns_working_bus() {
    use futures::executor::block_on;
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    block_on(async move {
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(())
        );
    });
}

/// @covers: noop_event_bus
#[test]
fn test_noop_event_bus_factory_returns_working_bus() {
    use futures::executor::block_on;
    let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
    block_on(async move {
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(())
        );
    });
}

/// @covers: reconstitute
#[test]
fn test_reconstitute_returns_none_for_unknown_id() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    #[derive(Default)]
    struct AnyAgg {
        id: String,
    }
    impl edge_application::Aggregate for AnyAgg {
        type Event = AnyEvent;
        fn apply(
            &mut self,
            req: AggregateApplyRequest<'_, AnyEvent>,
        ) -> Result<AggregateApplyResponse, EventError> {
            self.id = req
                .event
                .aggregate_id(EventAggregateIdRequest)
                .unwrap()
                .aggregate_id
                .into();
            Ok(AggregateApplyResponse)
        }
        fn id(
            &self,
            _req: AggregateIdentityRequest,
        ) -> Result<AggregateIdentityResponse<'_>, EventError> {
            Ok(AggregateIdentityResponse { id: &self.id })
        }
    }
    let store: Arc<dyn edge_application::EventStore<Event = AnyEvent>> =
        Domain.new_in_memory_event_store::<AnyEvent>();
    let result = rt
        .block_on(Domain.reconstitute::<AnyAgg>(&*store, "none"))
        .unwrap();
    assert!(result.is_none());
}

/// @covers: reconstitute
#[tokio::test]
async fn test_reconstitute_returns_none_for_empty_store() {
    #[derive(Default)]
    struct AnyAgg {
        id: String,
    }
    impl edge_application::Aggregate for AnyAgg {
        type Event = AnyEvent;
        fn apply(
            &mut self,
            req: AggregateApplyRequest<'_, AnyEvent>,
        ) -> Result<AggregateApplyResponse, EventError> {
            self.id = req
                .event
                .aggregate_id(EventAggregateIdRequest)
                .unwrap()
                .aggregate_id
                .into();
            Ok(AggregateApplyResponse)
        }
        fn id(
            &self,
            _req: AggregateIdentityRequest,
        ) -> Result<AggregateIdentityResponse<'_>, EventError> {
            Ok(AggregateIdentityResponse { id: &self.id })
        }
    }
    let store = Domain.new_in_memory_event_store::<AnyEvent>();
    let result = Domain
        .reconstitute::<AnyAgg>(&*store, "none")
        .await
        .unwrap();
    assert!(result.is_none());
}
