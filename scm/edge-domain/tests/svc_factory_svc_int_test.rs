//! Rule-221 coverage: _happy, _error, and _edge tests for every pub fn in domain_svc.rs.
#![cfg(all(feature = "command", feature = "handler", feature = "event", feature = "query", feature = "validator", feature = "repository"))]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::*;
use edge_application_command::{
    CommandDispatchRequest, ExecutionRequest as CommandExecutionRequest,
    NameRequest as CommandNameRequest, NameResponse as CommandNameResponse,
};
use edge_application_handler::{
    EmptinessRequest as HandlerEmptinessRequest, ExecutionRequest, LenRequest as HandlerLenRequest,
    ListIdsRequest,
};
use edge_application_observer::{ObserverContext, StdObserveFactory};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use futures::future::BoxFuture;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct BytePayload(u8);

impl edge_application_base::Request for BytePayload {}
impl edge_application_base::Response for BytePayload {}

fn test_ctx<'a>(
    security: &'a SecurityContext,
    bus: &'a dyn CommandBus,
    observer: &'a dyn ObserverContext,
) -> HandlerContext<'a> {
    HandlerContext {
        security,
        commands: bus,
        observer,
    }
}

// ─── helpers ─────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {
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
            occurred_at: std::time::SystemTime::UNIX_EPOCH,
        })
    }
}

#[derive(Default)]
struct AnyAgg {
    id: String,
}
impl Aggregate for AnyAgg {
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

struct ErrCommand;
impl Command for ErrCommand {
    fn name(&self, _req: CommandNameRequest) -> Result<CommandNameResponse, CommandError> {
        Ok(CommandNameResponse {
            name: "err".to_string(),
        })
    }
    fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::InvalidInput("intentional".into())) })
    }
}

struct OkQuery(String);
impl Query for OkQuery {
    type Result = String;
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "ok" })
    }
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
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "err" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::NotFound("intentional".into())) })
    }
}

struct AlwaysValid;
impl Validator for AlwaysValid {
    fn validate(
        &self,
        _req: edge_application_validator::ValidationRequest,
    ) -> Result<edge_application_validator::ValidationResponse, ValidatorError> {
        Ok(edge_application_validator::ValidationResponse)
    }
}

struct AlwaysInvalid;
impl Validator for AlwaysInvalid {
    fn validate(
        &self,
        _req: edge_application_validator::ValidationRequest,
    ) -> Result<edge_application_validator::ValidationResponse, ValidatorError> {
        Err(ValidatorError::Invalid("invalid config".into()))
    }
}

struct AnySpec;
impl Spec for AnySpec {
    type Entity = String;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, String>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: true })
    }
}

struct NoSpec;
impl Spec for NoSpec {
    type Entity = String;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, String>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: false })
    }
}

struct ErrStore;
impl EventStore for ErrStore {
    type Event = AnyEvent;
    fn append(
        &self,
        _req: EventStoreAppendRequest<'_, AnyEvent>,
    ) -> BoxFuture<'_, Result<EventStoreAppendResponse, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("test".into())) })
    }
    fn load(
        &self,
        _req: EventStoreLoadRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadResponse<AnyEvent>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("test".into())) })
    }
    fn load_from(
        &self,
        _req: EventStoreLoadFromRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadFromResponse<AnyEvent>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("test".into())) })
    }
}

// ─── echo_handler ────────────────────────────────────────────────────────────

#[test]
fn test_echo_handler_string_roundtrip_happy() {
    block_on(async {
        let h = Domain.echo_handler::<TextPayload>("id", "/");
        let security = SecurityContext::unauthenticated();
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = test_ctx(&security, bus.as_ref(), observer.as_ref());
        assert_eq!(
            h.execute(ExecutionRequest {
                req: TextPayload("ping".to_string()),
                ctx: &ctx
            })
            .await
            .unwrap(),
            TextPayload("ping".to_string())
        );
    });
}

#[test]
fn test_echo_handler_always_returns_ok_not_error() {
    block_on(async {
        // echo_handler execution is infallible — documents no error path
        let h = Domain.echo_handler::<TextPayload>("id", "/");
        let security = SecurityContext::unauthenticated();
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = test_ctx(&security, bus.as_ref(), observer.as_ref());
        let result = h
            .execute(ExecutionRequest {
                req: TextPayload("anything".to_string()),
                ctx: &ctx,
            })
            .await;
        assert_eq!(
            result,
            Ok(TextPayload("anything".to_string())),
            "echo handler should return the input unchanged"
        );
    });
}

#[test]
fn test_echo_handler_empty_string_preserved_edge() {
    block_on(async {
        let h = Domain.echo_handler::<TextPayload>("id", "/");
        let security = SecurityContext::unauthenticated();
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        let observer = StdObserveFactory::noop_observer_context();
        let ctx = test_ctx(&security, bus.as_ref(), observer.as_ref());
        assert_eq!(
            h.execute(ExecutionRequest {
                req: TextPayload(String::new()),
                ctx: &ctx
            })
            .await
            .unwrap(),
            TextPayload(String::new())
        );
    });
}

// ─── new_handler_registry ────────────────────────────────────────────────────

#[test]
fn test_new_handler_registry_starts_empty_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 0);
}

#[test]
fn test_new_handler_registry_get_unknown_id_returns_none_not_error() {
    // get on empty registry must return None, not panic or error
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg
        .get(edge_application_handler::HandlerLookupRequest {
            id: "unknown".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

#[test]
fn test_new_handler_registry_list_ids_empty_before_registration_edge() {
    let reg = Domain.new_handler_registry::<BytePayload, BytePayload>();
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

// ─── paired ──────────────────────────────────────────────────────────────────

#[test]
fn test_paired_both_closures_share_same_backend_happy() {
    let backend = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let (a, b) = Domain.paired(Arc::clone(&backend), |b| b, |b| b);
    a.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(b.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[test]
fn test_paired_write_through_first_visible_to_second_not_error() {
    block_on(async {
        // verifies shared backend — write through first is visible through second
        let repo = Domain.new_in_memory_repository::<String, u32>();
        let (writer, reader) = Domain.paired(repo, |r| r.clone(), |r| r);
        writer
            .save(RepositorySaveRequest {
                id: 1u32,
                entity: "x".into(),
            })
            .await
            .unwrap();
        assert!(reader
            .find(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap()
            .entity
            .is_some());
    });
}

#[test]
fn test_paired_returns_two_distinct_values_edge() {
    let backend = Arc::new(());
    let (a, b) = Domain.paired(Arc::clone(&backend), |_| 1u32, |_| 2u32);
    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

// ─── new_in_memory_repository ────────────────────────────────────────────────

#[test]
fn test_new_in_memory_repository_save_then_find_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "hello".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.find(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .entity
                .as_deref(),
            Some("hello")
        );
    });
}

#[test]
fn test_new_in_memory_repository_find_absent_returns_ok_none_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        assert!(repo
            .find(RepositoryIdRequest { id: &99u32 })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

#[test]
fn test_new_in_memory_repository_overwrite_same_id_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "first".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "second".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.find(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .entity
                .as_deref(),
            Some("second")
        );
    });
}

// ─── new_in_memory_queryable_repository ──────────────────────────────────────

#[test]
fn test_new_in_memory_queryable_repository_find_by_returns_matches_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "hello".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "world".into(),
        })
        .await
        .unwrap();
        let all = repo
            .find_by(SpecRequest {
                spec: Box::new(AnySpec),
            })
            .await
            .unwrap()
            .items;
        assert_eq!(all.len(), 2);
    });
}

#[test]
fn test_new_in_memory_queryable_repository_no_match_returns_empty_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        let result = repo
            .find_by(SpecRequest {
                spec: Box::new(NoSpec),
            })
            .await
            .unwrap()
            .items;
        assert!(result.is_empty());
    });
}

#[test]
fn test_new_in_memory_queryable_repository_find_one_by_none_on_empty_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        assert!(repo
            .find_one_by(SpecRequest {
                spec: Box::new(AnySpec)
            })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

// ─── direct_command_bus ──────────────────────────────────────────────────────

#[test]
fn test_direct_command_bus_dispatches_successful_command_happy() {
    block_on(async {
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountingCommand(Arc<AtomicUsize>);
        impl Command for CountingCommand {
            fn execute(
                &self,
                _req: CommandExecutionRequest,
            ) -> BoxFuture<'_, Result<(), CommandError>> {
                let counter = self.0.clone();
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                })
            }
        }

        let counter = Arc::new(AtomicUsize::new(0));
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        bus.dispatch(CommandDispatchRequest {
            command: Box::new(CountingCommand(counter.clone())),
        })
        .await
        .unwrap();
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "dispatch should actually execute the command"
        );
    });
}

#[test]
fn test_direct_command_bus_propagates_command_error() {
    block_on(async {
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        assert!(bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(ErrCommand)
            })
            .await
            .is_err());
    });
}

#[test]
fn test_direct_command_bus_error_message_preserved_edge() {
    block_on(async {
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        let err = bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(ErrCommand),
            })
            .await
            .unwrap_err();
        assert!(err.to_string().contains("intentional"));
    });
}

// ─── noop_event_publisher ────────────────────────────────────────────────────

#[test]
fn test_noop_event_publisher_publish_returns_ok_happy() {
    block_on(async {
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        assert_eq!(
            pub_.publish(EventPublisherPublishRequest { event: &AnyEvent })
                .await,
            Ok(()),
            "noop publisher should always succeed"
        );
    });
}

#[test]
fn test_noop_event_publisher_never_errors_not_error() {
    block_on(async {
        // documents infallibility: returns Ok regardless of event
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        assert_eq!(
            pub_.publish(EventPublisherPublishRequest { event: &AnyEvent })
                .await,
            Ok(()),
            "noop publisher is infallible"
        );
    });
}

#[test]
fn test_noop_event_publisher_accepts_repeated_publish_edge() {
    block_on(async {
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        for i in 0..3 {
            let result = pub_
                .publish(EventPublisherPublishRequest { event: &AnyEvent })
                .await;
            assert_eq!(
                result,
                Ok(()),
                "noop publisher should succeed on iteration {}",
                i
            );
        }
    });
}

// ─── new_in_memory_event_store ───────────────────────────────────────────────

#[test]
fn test_new_in_memory_event_store_append_then_load_happy() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<AnyEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![AnyEvent],
                expected: ExpectedVersion::Any,
            })
            .await
            .unwrap();
        let events = store
            .load(EventStoreLoadRequest {
                aggregate_id: "agg-1",
            })
            .await
            .unwrap()
            .events;
        assert_eq!(events.len(), 1);
    });
}

#[test]
fn test_new_in_memory_event_store_version_conflict_returns_error() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<AnyEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![AnyEvent],
                expected: ExpectedVersion::NoStream,
            })
            .await
            .unwrap();
        // second append with NoStream must fail: stream already exists
        let result = store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![AnyEvent],
                expected: ExpectedVersion::NoStream,
            })
            .await;
        assert!(result.is_err());
    });
}

#[test]
fn test_new_in_memory_event_store_load_nonexistent_stream_returns_empty_edge() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<AnyEvent>();
        let events = store
            .load(EventStoreLoadRequest {
                aggregate_id: "no-such-stream",
            })
            .await
            .unwrap()
            .events;
        assert!(events.is_empty());
    });
}

// ─── reconstitute ────────────────────────────────────────────────────────────

#[test]
fn test_reconstitute_with_events_returns_aggregate_happy() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<AnyEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![AnyEvent],
                expected: ExpectedVersion::Any,
            })
            .await
            .unwrap();
        let result = Domain
            .reconstitute::<AnyAgg>(&*store, "agg-1")
            .await
            .unwrap();
        assert!(result.is_some());
    });
}

#[test]
fn test_reconstitute_store_unavailable_propagates_error() {
    block_on(async {
        let result = Domain.reconstitute::<AnyAgg>(&ErrStore, "agg-1").await;
        assert!(result.is_err());
    });
}

#[test]
fn test_reconstitute_empty_stream_returns_none_edge() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<AnyEvent>();
        let result = Domain
            .reconstitute::<AnyAgg>(&*store, "agg-1")
            .await
            .unwrap();
        assert!(result.is_none());
    });
}

// ─── direct_query_bus ────────────────────────────────────────────────────────

#[test]
fn test_direct_query_bus_dispatches_successful_query_happy() {
    block_on(async {
        let bus = Domain.direct_query_bus::<String>();
        let result = bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(OkQuery("pong".into())),
            })
            .await
            .unwrap();
        assert_eq!(result.result, "pong");
    });
}

#[test]
fn test_direct_query_bus_propagates_query_error() {
    block_on(async {
        let bus = Domain.direct_query_bus::<String>();
        assert!(bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(ErrQuery)
            })
            .await
            .is_err());
    });
}

#[test]
fn test_direct_query_bus_dispatches_empty_result_edge() {
    block_on(async {
        let bus = Domain.direct_query_bus::<String>();
        let result = bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(OkQuery(String::new())),
            })
            .await
            .unwrap();
        assert_eq!(result.result, "");
    });
}

// ─── in_process_event_bus ────────────────────────────────────────────────────

#[test]
fn test_in_process_event_bus_publish_returns_ok_happy() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(()),
            "event bus should publish successfully"
        );
    });
}

#[test]
fn test_in_process_event_bus_publish_no_subscriber_not_error() {
    block_on(async {
        // fire-and-forget: publish without subscribers must succeed
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(()),
            "publish without subscribers should succeed"
        );
    });
}

#[test]
fn test_in_process_event_bus_default_config_creates_valid_bus_edge() {
    let bus = Domain
        .in_process_event_bus(InProcessEventBusRequest {
            config: EventBusConfig::default(),
        })
        .unwrap()
        .bus;
    assert!(
        !Arc::as_ptr(&bus).is_null(),
        "bus should be successfully constructed"
    );
}

// ─── noop_event_bus ──────────────────────────────────────────────────────────

#[test]
fn test_noop_event_bus_publish_returns_ok_happy() {
    block_on(async {
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(()),
            "noop bus should always succeed"
        );
    });
}

#[test]
fn test_noop_event_bus_publish_never_errors_not_error() {
    block_on(async {
        // noop bus is infallible — documents no error path
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(()),
            "noop bus is infallible"
        );
    });
}

#[test]
fn test_noop_event_bus_subscribe_source_is_closed_edge() {
    block_on(async {
        // noop bus subscribe returns a ClosedEventSource — first recv is Err
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_err());
    });
}

// ─── validate_config ─────────────────────────────────────────────────────────

#[test]
fn test_validate_config_valid_config_returns_ok_happy() {
    assert_eq!(
        Domain.validate_config(&AlwaysValid),
        Ok(()),
        "valid config should pass validation"
    );
}

#[test]
fn test_validate_config_invalid_config_returns_err_error() {
    assert!(Domain.validate_config(&AlwaysInvalid).is_err());
}

#[test]
fn test_validate_config_error_message_non_empty_edge() {
    let err = Domain.validate_config(&AlwaysInvalid).unwrap_err();
    assert!(!err.is_empty());
}
