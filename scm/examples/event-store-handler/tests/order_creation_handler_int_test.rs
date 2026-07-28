//! Integration test: `OrderCreationHandler::execute()` genuinely reads `HandlerContext` for
//! real observability and, as part of the same call, appends through a real,
//! constructor-injected `EventStore` — `Handler` -> `EventStore`, connected and working, as one
//! observable call chain. Assertions verify the event landed in the shared store, not just that
//! `execute()` returned `Ok`.
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
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
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
struct OrderCreationRequest {
    order_id: String,
    item: String,
}
impl edge_application_base::Request for OrderCreationRequest {}

#[derive(Debug, Clone, Copy)]
struct OrderCreationResponse {
    sequence: u64,
}
impl edge_application_base::Response for OrderCreationResponse {}

/// Holds its own injected `EventStore`; genuinely reads `req.ctx` inside `execute()`.
struct OrderCreationHandler {
    event_store: Arc<dyn EventStore<Event = OrderCreated>>,
}

#[async_trait]
impl Handler for OrderCreationHandler {
    type Request = OrderCreationRequest;
    type Response = OrderCreationResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderCreationRequest>,
    ) -> Result<OrderCreationResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "order_creation_handler".to_string(),
                message: format!("appending order.created for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let result = self
            .event_store
            .append(EventStoreAppendRequest {
                aggregate_id: &req.req.order_id,
                events: vec![OrderCreated {
                    order_id: req.req.order_id.clone(),
                    item: req.req.item,
                }],
                expected: ExpectedVersion::Any,
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(OrderCreationResponse {
            sequence: result.sequence,
        })
    }
}

fn build_handler() -> (
    OrderCreationHandler,
    Arc<dyn EventStore<Event = OrderCreated>>,
) {
    let event_store: Arc<dyn EventStore<Event = OrderCreated>> =
        Arc::new(MemoryEventStore::<OrderCreated>::new());
    let handler = OrderCreationHandler {
        event_store: Arc::clone(&event_store),
    };
    (handler, event_store)
}

fn run(
    handler: &OrderCreationHandler,
    order_id: &str,
    item: &str,
) -> Result<OrderCreationResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: OrderCreationRequest {
            order_id: order_id.to_string(),
            item: item.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_new_event_persists_in_event_store_happy() {
    let (handler, event_store) = build_handler();
    let response = run(&handler, "order-1", "widget").unwrap();
    assert_eq!(response.sequence, 1);

    let loaded = block_on(event_store.load(EventStoreLoadRequest {
        aggregate_id: "order-1",
    }))
    .unwrap();
    assert_eq!(loaded.events.len(), 1);
    assert_eq!(loaded.events[0].event.item, "widget");
    assert_eq!(loaded.events[0].sequence, 1);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_order_id_still_persists_error() {
    let (handler, event_store) = build_handler();
    let response = run(&handler, "", "mystery-item").unwrap();
    assert_eq!(response.sequence, 1);

    let loaded = block_on(event_store.load(EventStoreLoadRequest { aggregate_id: "" })).unwrap();
    assert_eq!(loaded.events.len(), 1);
}

/// @covers: Handler::execute
#[test]
fn test_execute_same_aggregate_twice_appends_sequentially_edge() {
    let (handler, event_store) = build_handler();
    let first = run(&handler, "order-2", "first-item").unwrap();
    let second = run(&handler, "order-2", "second-item").unwrap();
    assert_eq!(first.sequence, 1);
    assert_eq!(second.sequence, 2);

    let loaded = block_on(event_store.load(EventStoreLoadRequest {
        aggregate_id: "order-2",
    }))
    .unwrap();
    assert_eq!(loaded.events.len(), 2);
    assert_eq!(loaded.events[0].event.item, "first-item");
    assert_eq!(loaded.events[1].event.item, "second-item");
}
