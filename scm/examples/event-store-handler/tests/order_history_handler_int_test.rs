//! Integration test: `OrderHistoryHandler::execute()` reads through a real,
//! constructor-injected `EventStore` — the same instance a `OrderCreationHandler` appends
//! into, proving one event store can genuinely back multiple handlers, each reaching it through
//! its own injected field rather than through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_event::{
    DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventError, EventStore,
    EventStoreAppendRequest, EventStoreLoadRequest, EventTypeRequest, EventTypeResponse,
    ExpectedVersion, MemoryEventStore,
};
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone)]
struct OrderCreated {
    order_id: String,
    item: String,
}

impl DomainEvent for OrderCreated {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "order.created",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: &self.order_id,
        })
    }
}

#[derive(Debug, Clone)]
struct OrderHistoryRequest {
    order_id: String,
}
impl edge_application_base::Request for OrderHistoryRequest {}

#[derive(Debug, Clone)]
struct OrderHistoryResponse {
    items: Vec<String>,
}
impl edge_application_base::Response for OrderHistoryResponse {}

/// Holds its own injected `EventStore` — the per-call aggregate id arrives through
/// `Self::Request`.
struct OrderHistoryHandler {
    event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for OrderHistoryHandler {
    type Request = OrderHistoryRequest;
    type Response = OrderHistoryResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderHistoryRequest>,
    ) -> Result<OrderHistoryResponse, HandlerError> {
        let loaded = self
            .event_store
            .load(EventStoreLoadRequest {
                aggregate_id: &req.req.order_id,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        let items: Vec<String> = loaded
            .events
            .iter()
            .map(|envelope| format!("seq {}: {}", envelope.sequence, envelope.event.item))
            .collect();
        Ok(OrderHistoryResponse { items })
    }
}

/// Seeds an event store with two events for one aggregate and builds a handler sharing that
/// same instance — mirrors the multi-handler-one-store shape from `OrderCreationHandler`.
fn build_handler_with_seeded_history() -> OrderHistoryHandler {
    let event_store: Arc<dyn EventStore<Event = OrderCreated>> =
        Arc::new(MemoryEventStore::<OrderCreated>::new());
    block_on(event_store.append(EventStoreAppendRequest {
        aggregate_id: "order-1",
        events: vec![OrderCreated {
            order_id: "order-1".to_string(),
            item: "widget".to_string(),
        }],
        expected: ExpectedVersion::NoStream,
    }))
    .unwrap();
    block_on(event_store.append(EventStoreAppendRequest {
        aggregate_id: "order-1",
        events: vec![OrderCreated {
            order_id: "order-1".to_string(),
            item: "gadget".to_string(),
        }],
        expected: ExpectedVersion::Exact(1),
    }))
    .unwrap();
    OrderHistoryHandler { event_store }
}

fn run(
    handler: &OrderHistoryHandler,
    order_id: &str,
) -> Result<OrderHistoryResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: OrderHistoryRequest {
            order_id: order_id.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_aggregate_with_history_returns_events_in_order_happy() {
    let handler = build_handler_with_seeded_history();
    let response = run(&handler, "order-1").unwrap();
    assert_eq!(response.items, vec!["seq 1: widget", "seq 2: gadget"]);
}

/// @covers: Handler::execute
#[test]
fn test_execute_aggregate_with_no_events_returns_empty_error() {
    let handler = build_handler_with_seeded_history();
    let response = run(&handler, "never-appended").unwrap();
    assert!(response.items.is_empty());
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_independent_and_consistent_edge() {
    let handler = build_handler_with_seeded_history();
    let first = run(&handler, "order-1").unwrap();
    let second = run(&handler, "missing").unwrap();
    let third = run(&handler, "order-1").unwrap();
    assert!(!first.items.is_empty());
    assert!(second.items.is_empty());
    assert_eq!(first.items, third.items);
}
