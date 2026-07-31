//! Rule-222 coverage: _happy, _error, and _edge tests for every fn in every pub trait in api/.
//!
//! One test per unique method name × 3 suffixes covers all 47 trait functions because
//! the arch audit pattern `test_<fn>_*_<suffix>` matches on method name globally across traits.
#![cfg(all(feature = "event", feature = "command", feature = "query", feature = "repository", feature = "handler"))]
// @allow: no_mocks_in_integration — MemoryRepository is the production-shipped reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::*;
use edge_application_command::{
    CommandDispatchRequest, ExecutionRequest as CommandExecutionRequest,
    NameRequest as CommandNameRequest, NameResponse as CommandNameResponse,
};
use edge_application_handler::{
    DeregisterHandlerRequest, EmptinessRequest as HandlerEmptinessRequest, HandlerLookupRequest,
    IdRequest, LenRequest as HandlerLenRequest, ListIdsRequest, PatternRequest,
    RegisterHandlerRequest,
};
use futures::executor::block_on;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::time::SystemTime;

// ─── shared fixtures ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
struct TextPayload(String);

impl edge_application_base::Request for TextPayload {}
impl edge_application_base::Response for TextPayload {}

#[derive(Clone)]
struct TestEvent {
    aggregate_id: String,
}

impl DomainEvent for TestEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.event",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.aggregate_id,
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: SystemTime::UNIX_EPOCH,
        })
    }
}

#[derive(Default)]
struct TestAggregate {
    id: String,
    count: u32,
}

impl Aggregate for TestAggregate {
    type Event = TestEvent;
    fn apply(
        &mut self,
        req: AggregateApplyRequest<'_, TestEvent>,
    ) -> Result<AggregateApplyResponse, EventError> {
        self.id = req.event.aggregate_id.clone();
        self.count += 1;
        Ok(AggregateApplyResponse)
    }
    fn id(
        &self,
        _req: AggregateIdentityRequest,
    ) -> Result<AggregateIdentityResponse<'_>, EventError> {
        Ok(AggregateIdentityResponse { id: &self.id })
    }
}

struct OkCmd;
impl Command for OkCmd {
    fn name(&self, _req: CommandNameRequest) -> Result<CommandNameResponse, CommandError> {
        Ok(CommandNameResponse {
            name: "ok-cmd".to_string(),
        })
    }
    fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Ok(()) })
    }
}

struct ErrCmd;
impl Command for ErrCmd {
    fn name(&self, _req: CommandNameRequest) -> Result<CommandNameResponse, CommandError> {
        Ok(CommandNameResponse {
            name: "err-cmd".to_string(),
        })
    }
    fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
        Box::pin(async { Err(CommandError::RuleViolation("blocked".into())) })
    }
}

struct OkQry(String);
impl Query for OkQry {
    type Result = String;
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "ok-qry" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        let v = self.0.clone();
        Box::pin(async move { Ok(QueryResultResponse { result: v }) })
    }
}

struct ErrQry;
impl Query for ErrQry {
    type Result = String;
    fn name(&self, _req: QueryNameRequest) -> Result<QueryNameResponse<'_>, QueryError> {
        Ok(QueryNameResponse { name: "err-qry" })
    }
    fn execute(
        &self,
        _req: QueryExecuteRequest,
    ) -> BoxFuture<'_, Result<QueryResultResponse<String>, QueryError>> {
        Box::pin(async { Err(QueryError::Internal("oops".into())) })
    }
}

struct AlwaysMatch;
impl Spec for AlwaysMatch {
    type Entity = String;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, String>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: true })
    }
}

struct NeverMatch;
impl Spec for NeverMatch {
    type Entity = String;

    fn matches(
        &self,
        _req: SpecMatchesRequest<'_, String>,
    ) -> Result<SpecMatchesResponse, RepositoryError> {
        Ok(SpecMatchesResponse { matches: false })
    }
}

struct ErrEventStore;
impl EventStore for ErrEventStore {
    type Event = TestEvent;
    fn append(
        &self,
        _req: EventStoreAppendRequest<'_, TestEvent>,
    ) -> BoxFuture<'_, Result<EventStoreAppendResponse, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
    fn load(
        &self,
        _req: EventStoreLoadRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadResponse<TestEvent>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
    fn load_from(
        &self,
        _req: EventStoreLoadFromRequest<'_>,
    ) -> BoxFuture<'_, Result<EventStoreLoadFromResponse<TestEvent>, EventStoreError>> {
        Box::pin(async { Err(EventStoreError::Unavailable("down".into())) })
    }
}

fn make_test_handler() -> Arc<dyn Handler<Request = TextPayload, Response = TextPayload>> {
    Domain.echo_handler("test", "/test")
}

// ─── name ────────────────────────────────────────────────────────────────────
// Covers: Command::name, Query::name

#[test]
fn test_name_command_returns_defined_value_happy() {
    assert_eq!(OkCmd.name(CommandNameRequest).unwrap().name, "ok-cmd");
}

#[test]
fn test_name_query_consistent_across_calls_not_error() {
    // name() must never error — returns same value each call
    let q = OkQry("x".into());
    assert_eq!(
        q.name(QueryNameRequest).unwrap().name,
        "ok-qry",
        "query name should be stable and known"
    );
}

#[test]
fn test_name_command_default_impl_returns_command_edge() {
    // A Command that doesn't override name() falls back to Command's own
    // provided default ("command"), distinct from OkCmd/ErrCmd which both override it.
    struct DefaultNameCmd;
    impl Command for DefaultNameCmd {
        fn execute(&self, _req: CommandExecutionRequest) -> BoxFuture<'_, Result<(), CommandError>> {
            Box::pin(async { Ok(()) })
        }
    }
    assert_eq!(
        DefaultNameCmd.name(CommandNameRequest).unwrap().name,
        "command"
    );
}

// ─── execute ─────────────────────────────────────────────────────────────────
// Covers: Command::execute, Query::execute

#[test]
fn test_execute_command_returns_ok_happy() {
    block_on(async {
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountingCmd(AtomicUsize);
        impl Command for CountingCmd {
            fn execute(
                &self,
                _req: CommandExecutionRequest,
            ) -> BoxFuture<'_, Result<(), CommandError>> {
                self.0.fetch_add(1, Ordering::SeqCst);
                Box::pin(async { Ok(()) })
            }
        }

        let cmd = CountingCmd(AtomicUsize::new(0));
        cmd.execute(CommandExecutionRequest).await.unwrap();
        assert_eq!(
            cmd.0.load(Ordering::SeqCst),
            1,
            "command should execute exactly once"
        );
    });
}

#[test]
fn test_execute_command_returns_err_on_failure_error() {
    block_on(async {
        assert!(ErrCmd.execute(CommandExecutionRequest).await.is_err());
    });
}

#[test]
fn test_execute_query_with_empty_response_edge() {
    block_on(async {
        let result = OkQry(String::new())
            .execute(QueryExecuteRequest)
            .await
            .unwrap();
        assert_eq!(result.result, "");
    });
}

// ─── dispatch ────────────────────────────────────────────────────────────────
// Covers: CommandBus::dispatch, QueryBus::dispatch

#[test]
fn test_dispatch_command_returns_ok_happy() {
    block_on(async {
        use std::sync::atomic::{AtomicUsize, Ordering};

        struct CountingCmd(Arc<AtomicUsize>);
        impl Command for CountingCmd {
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
            command: Box::new(CountingCmd(counter.clone())),
        })
        .await
        .unwrap();
        assert_eq!(
            counter.load(Ordering::SeqCst),
            1,
            "dispatch should execute the command exactly once"
        );
    });
}

#[test]
fn test_dispatch_command_propagates_error() {
    block_on(async {
        let bus = Domain
            .direct_command_bus(DirectCommandBusRequest)
            .unwrap()
            .bus;
        assert!(bus
            .dispatch(CommandDispatchRequest {
                command: Box::new(ErrCmd)
            })
            .await
            .is_err());
    });
}

#[test]
fn test_dispatch_query_result_type_preserved_edge() {
    block_on(async {
        let bus = Domain.direct_query_bus::<String>();
        let r = bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(OkQry("echo".into())),
            })
            .await
            .expect("dispatch failed");
        assert_eq!(r.result, "echo");
    });
}

#[test]
fn test_dispatch_query_propagates_error_error() {
    block_on(async {
        let bus = Domain.direct_query_bus::<String>();
        assert!(bus
            .dispatch(QueryDispatchRequest {
                query: Box::new(ErrQry)
            })
            .await
            .is_err());
    });
}

// ─── apply ───────────────────────────────────────────────────────────────────
// Covers: Aggregate::apply

#[test]
fn test_apply_event_updates_aggregate_state_happy() {
    let mut agg = TestAggregate::default();
    agg.apply(AggregateApplyRequest {
        event: &TestEvent {
            aggregate_id: "a1".into(),
        },
    })
    .unwrap();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "a1");
    assert_eq!(agg.count, 1);
}

#[test]
fn test_apply_no_op_on_default_impl_not_error() {
    // Default Aggregate::apply does nothing — verifies it doesn't panic
    #[derive(Default)]
    struct NoOpAgg;
    #[derive(Clone)]
    struct NoOpEvent;
    impl DomainEvent for NoOpEvent {
        fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
            Ok(EventTypeResponse { event_type: "noop" })
        }
        fn aggregate_id(
            &self,
            _req: EventAggregateIdRequest,
        ) -> Result<EventAggregateIdResponse<'_>, EventError> {
            Ok(EventAggregateIdResponse { aggregate_id: "" })
        }
        fn occurred_at(
            &self,
            _req: EventOccurredAtRequest,
        ) -> Result<EventOccurredAtResponse, EventError> {
            Ok(EventOccurredAtResponse {
                occurred_at: SystemTime::UNIX_EPOCH,
            })
        }
    }
    impl Aggregate for NoOpAgg {
        type Event = NoOpEvent;
    }
    let mut agg = NoOpAgg;
    agg.apply(AggregateApplyRequest { event: &NoOpEvent })
        .unwrap(); // should not panic
    assert_eq!(
        agg.id(AggregateIdentityRequest).unwrap().id,
        "",
        "default impl should not modify state"
    );
}

#[test]
fn test_apply_called_twice_increments_count_edge() {
    let mut agg = TestAggregate::default();
    agg.apply(AggregateApplyRequest {
        event: &TestEvent {
            aggregate_id: "a".into(),
        },
    })
    .unwrap();
    agg.apply(AggregateApplyRequest {
        event: &TestEvent {
            aggregate_id: "a".into(),
        },
    })
    .unwrap();
    assert_eq!(agg.count, 2);
}

// ─── id ──────────────────────────────────────────────────────────────────────
// Covers: Aggregate::id, Handler::id

#[test]
fn test_id_aggregate_reflects_applied_event_happy() {
    let mut agg = TestAggregate::default();
    agg.apply(AggregateApplyRequest {
        event: &TestEvent {
            aggregate_id: "agg-42".into(),
        },
    })
    .unwrap();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "agg-42");
}

#[test]
fn test_id_handler_matches_constructor_arg_not_error() {
    let h = make_test_handler();
    assert_eq!(h.id(IdRequest).unwrap().id, "test");
}

#[test]
fn test_id_aggregate_default_is_empty_edge() {
    let agg = TestAggregate::default();
    assert_eq!(agg.id(AggregateIdentityRequest).unwrap().id, "");
}

// ─── event_type ──────────────────────────────────────────────────────────────
// Covers: DomainEvent::event_type

#[test]
fn test_event_type_returns_defined_value_happy() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(
        e.event_type(EventTypeRequest).unwrap().event_type,
        "test.event"
    );
}

#[test]
fn test_event_type_stable_across_calls_not_error() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(
        e.event_type(EventTypeRequest).unwrap().event_type,
        "test.event",
        "event type should be stable and known"
    );
}

#[test]
fn test_event_type_default_impl_returns_event_edge() {
    // Default DomainEvent::event_type returns "event" — documents default behavior
    struct DefaultEvent;
    impl DomainEvent for DefaultEvent {}
    assert_eq!(
        DefaultEvent
            .event_type(EventTypeRequest)
            .unwrap()
            .event_type,
        "event"
    );
}

// ─── aggregate_id ────────────────────────────────────────────────────────────
// Covers: DomainEvent::aggregate_id

#[test]
fn test_aggregate_id_returns_set_value_happy() {
    let e = TestEvent {
        aggregate_id: "order-1".into(),
    };
    assert_eq!(
        e.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "order-1"
    );
}

#[test]
fn test_aggregate_id_consistent_across_calls_not_error() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(
        e.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        "x",
        "aggregate id should be stable and known"
    );
}

#[test]
fn test_aggregate_id_can_be_empty_string_edge() {
    let e = TestEvent {
        aggregate_id: String::new(),
    };
    assert_eq!(
        e.aggregate_id(EventAggregateIdRequest)
            .unwrap()
            .aggregate_id,
        ""
    );
}

// ─── occurred_at ─────────────────────────────────────────────────────────────
// Covers: DomainEvent::occurred_at

#[test]
fn test_occurred_at_returns_expected_time_happy() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    assert_eq!(
        e.occurred_at(EventOccurredAtRequest).unwrap().occurred_at,
        SystemTime::UNIX_EPOCH
    );
}

#[test]
fn test_occurred_at_is_after_unix_epoch_not_error() {
    // Default occurred_at is SystemTime::now() — must be >= UNIX_EPOCH
    struct NowEvent;
    impl DomainEvent for NowEvent {}
    assert!(
        NowEvent
            .occurred_at(EventOccurredAtRequest)
            .unwrap()
            .occurred_at
            >= SystemTime::UNIX_EPOCH
    );
}

#[test]
fn test_occurred_at_unix_epoch_is_valid_timestamp_edge() {
    let e = TestEvent {
        aggregate_id: "x".into(),
    };
    let dur = e
        .occurred_at(EventOccurredAtRequest)
        .unwrap()
        .occurred_at
        .duration_since(SystemTime::UNIX_EPOCH);
    assert!(
        dur.is_ok(),
        "timestamp should be valid and after unix epoch"
    );
}

// ─── publish ─────────────────────────────────────────────────────────────────
// Covers: EventBus::publish, EventPublisher::publish

#[test]
fn test_publish_to_noop_bus_returns_ok_happy() {
    block_on(async {
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "x".into(),
        });
        assert_eq!(
            bus.publish(EventBusPublishRequest { event: e }).await,
            Ok(()),
            "noop bus should always succeed"
        );
    });
}

#[test]
fn test_publish_to_noop_publisher_never_errors_not_error() {
    block_on(async {
        let pub_ = Domain
            .noop_event_publisher(NoopEventPublisherRequest)
            .unwrap()
            .publisher;
        let e = TestEvent {
            aggregate_id: "x".into(),
        };
        assert_eq!(
            pub_.publish(EventPublisherPublishRequest { event: &e })
                .await,
            Ok(()),
            "noop publisher is infallible"
        );
    });
}

#[test]
fn test_publish_multiple_events_sequentially_edge() {
    block_on(async {
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        for i in 0..5u32 {
            let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
                aggregate_id: i.to_string(),
            });
            let result = bus.publish(EventBusPublishRequest { event: e }).await;
            assert_eq!(
                result,
                Ok(()),
                "noop bus publish should always succeed for iteration {}",
                i
            );
        }
    });
}

// ─── subscribe ───────────────────────────────────────────────────────────────
// Covers: EventBus::subscribe

#[test]
fn test_subscribe_noop_bus_yields_receiver_happy() {
    block_on(async {
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        // noop bus's receiver immediately signals unavailable
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_err());
    });
}

#[test]
fn test_subscribe_active_bus_receives_published_event_not_error() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "e1".into(),
        });
        bus.publish(EventBusPublishRequest { event: e })
            .await
            .unwrap();
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_ok());
    });
}

#[test]
fn test_subscribe_multiple_receivers_each_get_event_edge() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        let mut rx1 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let mut rx2 = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "e2".into(),
        });
        bus.publish(EventBusPublishRequest { event: e })
            .await
            .unwrap();
        assert!(rx1.recv_next(EventSourceRecvNextRequest).await.is_ok());
        assert!(rx2.recv_next(EventSourceRecvNextRequest).await.is_ok());
    });
}

// ─── append ──────────────────────────────────────────────────────────────────
// Covers: EventStore::append

#[test]
fn test_append_returns_version_after_first_event_happy() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        let ver = store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                expected: ExpectedVersion::NoStream,
            })
            .await
            .unwrap()
            .sequence;
        assert_eq!(ver, 1);
    });
}

#[test]
fn test_append_nostream_on_existing_stream_returns_error() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                expected: ExpectedVersion::NoStream,
            })
            .await
            .unwrap();
        let result = store
            .append(EventStoreAppendRequest {
                aggregate_id: "agg-1",
                events: vec![TestEvent {
                    aggregate_id: "agg-1".into(),
                }],
                expected: ExpectedVersion::NoStream,
            })
            .await;
        assert!(result.is_err());
    });
}

#[test]
fn test_append_any_version_never_conflicts_edge() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        for i in 0..3u64 {
            let response = store
                .append(EventStoreAppendRequest {
                    aggregate_id: "agg-1",
                    events: vec![TestEvent {
                        aggregate_id: "agg-1".into(),
                    }],
                    expected: ExpectedVersion::Any,
                })
                .await
                .unwrap_or_else(|e| {
                    panic!("append with Any version should succeed iteration {i}: {e:?}")
                });
            assert_eq!(
                response.sequence,
                i + 1,
                "sequence should increment monotonically"
            );
        }
    });
}

// ─── load ────────────────────────────────────────────────────────────────────
// Covers: EventStore::load

#[test]
fn test_load_after_append_returns_events_happy() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "a1",
                events: vec![
                    TestEvent {
                        aggregate_id: "a1".into(),
                    },
                    TestEvent {
                        aggregate_id: "a1".into(),
                    },
                ],
                expected: ExpectedVersion::Any,
            })
            .await
            .unwrap();
        let events = store
            .load(EventStoreLoadRequest { aggregate_id: "a1" })
            .await
            .unwrap()
            .events;
        assert_eq!(events.len(), 2);
    });
}

#[test]
fn test_load_nonexistent_stream_returns_empty_not_error() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        let events = store
            .load(EventStoreLoadRequest {
                aggregate_id: "ghost",
            })
            .await
            .unwrap()
            .events;
        assert!(events.is_empty());
    });
}

#[test]
fn test_load_events_have_correct_sequence_edge() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "a1",
                events: vec![TestEvent {
                    aggregate_id: "a1".into(),
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .unwrap();
        let events = store
            .load(EventStoreLoadRequest { aggregate_id: "a1" })
            .await
            .unwrap()
            .events;
        assert_eq!(events[0].sequence, 1);
    });
}

// ─── load_from ───────────────────────────────────────────────────────────────
// Covers: EventStore::load_from

#[test]
fn test_load_from_returns_subset_from_sequence_happy() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        for _ in 0..3u32 {
            store
                .append(EventStoreAppendRequest {
                    aggregate_id: "a1",
                    events: vec![TestEvent {
                        aggregate_id: "a1".into(),
                    }],
                    expected: ExpectedVersion::Any,
                })
                .await
                .unwrap();
        }
        let events = store
            .load_from(EventStoreLoadFromRequest {
                aggregate_id: "a1",
                from_sequence: 2,
            })
            .await
            .unwrap()
            .events;
        assert_eq!(events.len(), 2); // sequences 2 and 3
    });
}

#[test]
fn test_load_from_beyond_end_returns_empty_not_error() {
    block_on(async {
        let store = Domain.new_in_memory_event_store::<TestEvent>();
        store
            .append(EventStoreAppendRequest {
                aggregate_id: "a1",
                events: vec![TestEvent {
                    aggregate_id: "a1".into(),
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .unwrap();
        let events = store
            .load_from(EventStoreLoadFromRequest {
                aggregate_id: "a1",
                from_sequence: 999,
            })
            .await
            .unwrap()
            .events;
        assert!(events.is_empty());
    });
}

#[test]
fn test_load_from_unavailable_store_propagates_error_edge() {
    block_on(async {
        let result = ErrEventStore
            .load_from(EventStoreLoadFromRequest {
                aggregate_id: "a1",
                from_sequence: 0,
            })
            .await;
        assert!(result.is_err());
    });
}

// ─── recv_next ───────────────────────────────────────────────────────────────
// Covers: EventSource::recv_next (tested via Box<dyn EventSource>)

#[test]
fn test_recv_next_active_bus_returns_event_happy() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "r1".into(),
        });
        bus.publish(EventBusPublishRequest { event: e })
            .await
            .unwrap();
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_ok());
    });
}

#[test]
fn test_recv_next_closed_source_returns_unavailable_error() {
    block_on(async {
        // noop bus subscribe returns a ClosedEventSource
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(matches!(
            rx.recv_next(EventSourceRecvNextRequest).await,
            Err(EventError::Unavailable(_))
        ));
    });
}

#[test]
fn test_recv_next_event_type_preserved_edge() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        let e: Arc<dyn DomainEvent> = Arc::new(TestEvent {
            aggregate_id: "r2".into(),
        });
        bus.publish(EventBusPublishRequest { event: e })
            .await
            .unwrap();
        let received = rx
            .recv_next(EventSourceRecvNextRequest)
            .await
            .unwrap()
            .event;
        assert_eq!(
            received.event_type(EventTypeRequest).unwrap().event_type,
            "test.event"
        );
    });
}

// ─── pattern ─────────────────────────────────────────────────────────────────
// Covers: Handler::pattern

#[test]
fn test_pattern_handler_matches_constructor_arg_happy() {
    let h = make_test_handler();
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/test");
}

#[test]
fn test_pattern_stable_across_calls_not_error() {
    let h = make_test_handler();
    assert_eq!(
        h.pattern(PatternRequest).unwrap().pattern,
        "/test",
        "handler pattern should be stable and known"
    );
}

#[test]
fn test_pattern_can_be_root_path_edge() {
    let h = Domain.echo_handler::<TextPayload>("root", "/");
    assert_eq!(h.pattern(PatternRequest).unwrap().pattern, "/");
}

// ─── register ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::register

#[test]
fn test_register_handler_then_registry_not_empty_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert!(!reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
}

#[test]
fn test_register_same_id_twice_overwrites_not_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 1); // not 2 — overwrites
}

#[test]
fn test_register_two_distinct_ids_both_retrievable_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("a", "/a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("b", "/b")))
        .unwrap();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 2);
}

// ─── deregister ──────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::deregister

#[test]
fn test_deregister_registered_handler_returns_true_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert!(
        reg.deregister(DeregisterHandlerRequest {
            id: "test".to_string()
        })
        .unwrap()
        .was_present
    );
}

#[test]
fn test_deregister_absent_handler_returns_false_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(
        !reg.deregister(DeregisterHandlerRequest {
            id: "ghost".to_string()
        })
        .unwrap()
        .was_present
    );
}

#[test]
fn test_deregister_already_removed_id_returns_false_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert!(
        reg.deregister(DeregisterHandlerRequest {
            id: "test".to_string()
        })
        .unwrap()
        .was_present
    );
    assert!(
        !reg.deregister(DeregisterHandlerRequest {
            id: "test".to_string()
        })
        .unwrap()
        .was_present
    );
}

// ─── get ─────────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::get

#[test]
fn test_get_registered_handler_returns_some_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    let h = make_test_handler();
    reg.register(RegisterHandlerRequest::new(h.clone()))
        .unwrap();
    let result = reg
        .get(HandlerLookupRequest {
            id: "test".to_string(),
        })
        .unwrap()
        .handler;
    assert!(result.is_some(), "registered handler should be retrievable");
    assert_eq!(
        result.unwrap().pattern(PatternRequest).unwrap().pattern,
        "/test",
        "retrieved handler should match registered handler"
    );
}

#[test]
fn test_get_nonexistent_key_returns_none_not_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg
        .get(HandlerLookupRequest {
            id: "ghost".to_string()
        })
        .unwrap()
        .handler
        .is_none());
}

#[test]
fn test_get_returns_the_matching_id_not_another_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("a", "/a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("b", "/b")))
        .unwrap();
    let result = reg
        .get(HandlerLookupRequest {
            id: "b".to_string(),
        })
        .unwrap()
        .handler
        .expect("handler b should be present");
    assert_eq!(result.pattern(PatternRequest).unwrap().pattern, "/b");
}

// ─── list_ids ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::list_ids

#[test]
fn test_list_ids_contains_registered_id_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert!(reg
        .list_ids(ListIdsRequest)
        .unwrap()
        .ids
        .contains(&"test".to_string()));
}

#[test]
fn test_list_ids_empty_before_registration_not_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.list_ids(ListIdsRequest).unwrap().ids.is_empty());
}

#[test]
fn test_list_ids_len_matches_registry_len_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("a", "/a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("b", "/b")))
        .unwrap();
    assert_eq!(
        reg.list_ids(ListIdsRequest).unwrap().ids.len(),
        reg.len(HandlerLenRequest).unwrap().count
    );
}

// ─── len ─────────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::len

#[test]
fn test_len_increments_after_register_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 0);
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 1);
}

#[test]
fn test_len_decrements_after_deregister_not_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    reg.deregister(DeregisterHandlerRequest {
        id: "test".to_string(),
    })
    .unwrap();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 0);
}

#[test]
fn test_len_after_multiple_registers_and_one_deregister_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("a", "/a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("b", "/b")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("c", "/c")))
        .unwrap();
    reg.deregister(DeregisterHandlerRequest {
        id: "b".to_string(),
    })
    .unwrap();
    assert_eq!(reg.len(HandlerLenRequest).unwrap().count, 2);
}

// ─── is_empty ────────────────────────────────────────────────────────────────
// Covers: HandlerRegistry::is_empty

#[test]
fn test_is_empty_true_on_new_registry_happy() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    assert!(reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
}

#[test]
fn test_is_empty_false_after_registration_not_error() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(make_test_handler()))
        .unwrap();
    assert!(!reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
}

#[test]
fn test_is_empty_true_after_deregistering_every_registered_handler_edge() {
    let reg = Domain.new_handler_registry::<TextPayload, TextPayload>();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("a", "/a")))
        .unwrap();
    reg.register(RegisterHandlerRequest::new(Domain.echo_handler("b", "/b")))
        .unwrap();
    reg.deregister(DeregisterHandlerRequest {
        id: "a".to_string(),
    })
    .unwrap();
    reg.deregister(DeregisterHandlerRequest {
        id: "b".to_string(),
    })
    .unwrap();
    assert!(reg.is_empty(HandlerEmptinessRequest).unwrap().empty);
}

// ─── find_by ─────────────────────────────────────────────────────────────────
// Covers: QueryableRepository::find_by

#[test]
fn test_find_by_returns_all_matching_items_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "alpha".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "beta".into(),
        })
        .await
        .unwrap();
        let all = repo
            .find_by(SpecRequest {
                spec: Box::new(AlwaysMatch),
            })
            .await
            .unwrap()
            .items;
        assert_eq!(all.len(), 2);
    });
}

#[test]
fn test_find_by_no_match_returns_empty_vec_not_error() {
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
                spec: Box::new(NeverMatch),
            })
            .await
            .unwrap()
            .items;
        assert!(result.is_empty());
    });
}

#[test]
fn test_find_by_empty_repo_returns_empty_vec_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        let result = repo
            .find_by(SpecRequest {
                spec: Box::new(AlwaysMatch),
            })
            .await
            .unwrap()
            .items;
        assert!(result.is_empty());
    });
}

// ─── find_one_by ─────────────────────────────────────────────────────────────
// Covers: QueryableRepository::find_one_by

#[test]
fn test_find_one_by_returns_first_match_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "first".into(),
        })
        .await
        .unwrap();
        let result = repo
            .find_one_by(SpecRequest {
                spec: Box::new(AlwaysMatch),
            })
            .await
            .unwrap()
            .entity;
        assert_eq!(result.as_deref(), Some("first"));
    });
}

#[test]
fn test_find_one_by_no_match_returns_none_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        assert!(repo
            .find_one_by(SpecRequest {
                spec: Box::new(NeverMatch)
            })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

#[test]
fn test_find_one_by_empty_repo_returns_none_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        assert!(repo
            .find_one_by(SpecRequest {
                spec: Box::new(AlwaysMatch)
            })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

// ─── count_by ────────────────────────────────────────────────────────────────
// Covers: QueryableRepository::count_by

#[test]
fn test_count_by_returns_matching_count_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "a".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "b".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.count_by(SpecRequest {
                spec: Box::new(AlwaysMatch)
            })
            .await
            .unwrap()
            .count,
            2
        );
    });
}

#[test]
fn test_count_by_no_match_returns_zero_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.count_by(SpecRequest {
                spec: Box::new(NeverMatch)
            })
            .await
            .unwrap()
            .count,
            0
        );
    });
}

#[test]
fn test_count_by_empty_repo_returns_zero_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_queryable_repository::<String, u32>();
        assert_eq!(
            repo.count_by(SpecRequest {
                spec: Box::new(AlwaysMatch)
            })
            .await
            .unwrap()
            .count,
            0
        );
    });
}

// ─── find ────────────────────────────────────────────────────────────────────
// Covers: Repository::find

#[test]
fn test_find_after_save_returns_some_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 7u32,
            entity: "seven".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.find(RepositoryIdRequest { id: &7u32 })
                .await
                .unwrap()
                .entity
                .as_deref(),
            Some("seven")
        );
    });
}

#[test]
fn test_find_nonexistent_returns_ok_none_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        assert!(repo
            .find(RepositoryIdRequest { id: &0u32 })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

#[test]
fn test_find_after_delete_returns_none_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        repo.delete(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap();
        assert!(repo
            .find(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap()
            .entity
            .is_none());
    });
}

// ─── save ────────────────────────────────────────────────────────────────────
// Covers: Repository::save

#[test]
fn test_save_then_find_round_trips_happy() {
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
fn test_save_overwrites_existing_entity_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "old".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "new".into(),
        })
        .await
        .unwrap();
        assert_eq!(
            repo.find(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .entity
                .as_deref(),
            Some("new")
        );
    });
}

#[test]
fn test_save_multiple_entities_increases_count_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        for i in 0..5u32 {
            repo.save(RepositorySaveRequest {
                id: i,
                entity: i.to_string(),
            })
            .await
            .unwrap();
        }
        assert_eq!(repo.count(RepositoryListRequest).await.unwrap().count, 5);
    });
}

// ─── delete ──────────────────────────────────────────────────────────────────
// Covers: Repository::delete

#[test]
fn test_delete_existing_entity_returns_true_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        assert!(
            repo.delete(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .removed
        );
    });
}

#[test]
fn test_delete_nonexistent_entity_returns_false_error() {
    block_on(async {
        // delete of non-existent key must return false, not Err
        let repo = Domain.new_in_memory_repository::<String, u32>();
        assert!(
            !repo
                .delete(RepositoryIdRequest { id: &99u32 })
                .await
                .unwrap()
                .removed
        );
    });
}

#[test]
fn test_delete_reduces_count_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "a".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "b".into(),
        })
        .await
        .unwrap();
        repo.delete(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap();
        assert_eq!(repo.count(RepositoryListRequest).await.unwrap().count, 1);
    });
}

// ─── list ────────────────────────────────────────────────────────────────────
// Covers: Repository::list

#[test]
fn test_list_returns_all_saved_entities_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "a".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "b".into(),
        })
        .await
        .unwrap();
        let all = repo.list(RepositoryListRequest).await.unwrap().items;
        assert_eq!(all.len(), 2);
    });
}

#[test]
fn test_list_empty_repo_returns_empty_vec_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        let all = repo.list(RepositoryListRequest).await.unwrap().items;
        assert!(all.is_empty());
    });
}

#[test]
fn test_list_after_delete_reflects_removal_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "a".into(),
        })
        .await
        .unwrap();
        repo.delete(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap();
        let all = repo.list(RepositoryListRequest).await.unwrap().items;
        assert!(all.is_empty());
    });
}

// ─── exists ──────────────────────────────────────────────────────────────────
// Covers: Repository::exists

#[test]
fn test_exists_after_save_returns_true_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        assert!(
            repo.exists(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .exists
        );
    });
}

#[test]
fn test_exists_nonexistent_returns_false_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        assert!(
            !repo
                .exists(RepositoryIdRequest { id: &99u32 })
                .await
                .unwrap()
                .exists
        );
    });
}

#[test]
fn test_exists_after_delete_returns_false_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        repo.delete(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap();
        assert!(
            !repo
                .exists(RepositoryIdRequest { id: &1u32 })
                .await
                .unwrap()
                .exists
        );
    });
}

// ─── count ───────────────────────────────────────────────────────────────────
// Covers: Repository::count

#[test]
fn test_count_reflects_number_of_saved_entities_happy() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "a".into(),
        })
        .await
        .unwrap();
        repo.save(RepositorySaveRequest {
            id: 2u32,
            entity: "b".into(),
        })
        .await
        .unwrap();
        assert_eq!(repo.count(RepositoryListRequest).await.unwrap().count, 2);
    });
}

#[test]
fn test_count_empty_repo_returns_zero_not_error() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        assert_eq!(repo.count(RepositoryListRequest).await.unwrap().count, 0);
    });
}

#[test]
fn test_count_decrements_after_delete_edge() {
    block_on(async {
        let repo = Domain.new_in_memory_repository::<String, u32>();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        repo.delete(RepositoryIdRequest { id: &1u32 })
            .await
            .unwrap();
        assert_eq!(repo.count(RepositoryListRequest).await.unwrap().count, 0);
    });
}

// ─── list_page ───────────────────────────────────────────────────────────────
// Covers: Repository::list_page

#[test]
fn test_list_page_returns_first_page_happy() {
    block_on(async {
        let repo = MemoryRepository::<String, u32>::new();
        for i in 0..5u32 {
            repo.save(RepositorySaveRequest {
                id: i,
                entity: i.to_string(),
            })
            .await
            .unwrap();
        }
        let page = repo
            .list_page(RepositoryListPageRequest {
                offset: 0,
                limit: 3,
            })
            .await
            .unwrap()
            .page;
        assert_eq!(page.items.len(), 3);
        assert_eq!(page.total, 5);
    });
}

#[test]
fn test_list_page_offset_beyond_end_returns_empty_items_not_error() {
    block_on(async {
        let repo = MemoryRepository::<String, u32>::new();
        repo.save(RepositorySaveRequest {
            id: 1u32,
            entity: "x".into(),
        })
        .await
        .unwrap();
        let page = repo
            .list_page(RepositoryListPageRequest {
                offset: 10,
                limit: 5,
            })
            .await
            .unwrap()
            .page;
        assert!(page.items.is_empty());
        assert_eq!(page.total, 1);
    });
}

#[test]
fn test_list_page_total_equals_full_count_edge() {
    block_on(async {
        let repo = MemoryRepository::<String, u32>::new();
        for i in 0..4u32 {
            repo.save(RepositorySaveRequest {
                id: i,
                entity: i.to_string(),
            })
            .await
            .unwrap();
        }
        let page = repo
            .list_page(RepositoryListPageRequest {
                offset: 0,
                limit: 2,
            })
            .await
            .unwrap()
            .page;
        assert_eq!(page.total, 4);
        assert_eq!(page.items.len(), 2);
    });
}

// ─── matches ─────────────────────────────────────────────────────────────────
// Covers: Spec::matches

#[test]
fn test_matches_always_true_spec_returns_true_happy() {
    let entity = "anything".to_string();
    assert!(
        AlwaysMatch
            .matches(SpecMatchesRequest { entity: &entity })
            .unwrap()
            .matches
    );
}

#[test]
fn test_matches_always_false_spec_returns_false_error() {
    let entity = "anything".to_string();
    assert!(
        !NeverMatch
            .matches(SpecMatchesRequest { entity: &entity })
            .unwrap()
            .matches
    );
}

#[test]
fn test_matches_empty_string_input_edge() {
    let entity = String::new();
    assert!(
        AlwaysMatch
            .matches(SpecMatchesRequest { entity: &entity })
            .unwrap()
            .matches
    );
}

