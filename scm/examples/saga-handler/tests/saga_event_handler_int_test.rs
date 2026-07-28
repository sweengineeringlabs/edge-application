//! Integration test: `SagaEventHandler::execute()` genuinely reads `HandlerContext` for
//! real observability and, as part of the same call, mutates a real, constructor-injected saga
//! through `Arc<Mutex<S>>` — `Handler` -> `Saga`, connected and working, as one observable call
//! chain. Assertions verify the saga's own state changed, not just that `execute()` returned
//! `Ok`.
//!
//! Wired via `Arc<Mutex<S>>` directly, not `SagaStore` — `SagaStore::get()` returns an
//! immutable borrow, which cannot support the get-then-`handle` mutation path `Saga::handle`
//! (`&mut self`) requires. See `examples/saga-handler/src/main.rs` for the full finding.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_saga::{
    Saga, SagaCommand, SagaCommandDispatchRequest, SagaError, SagaEvent,
    SagaEventDescribeRequest, SagaEventDescribeResponse, SagaHandleRequest, SagaHandleResponse,
    SagaIsCompleteRequest, SagaIsCompleteResponse,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use parking_lot::Mutex;

#[derive(Debug, Clone)]
struct OrderPaid {
    order_id: String,
}

impl SagaEvent for OrderPaid {
    fn describe(
        &self,
        _req: SagaEventDescribeRequest,
    ) -> Result<SagaEventDescribeResponse, SagaError> {
        Ok(SagaEventDescribeResponse {
            event_type: "order.paid".to_string(),
            aggregate_id: self.order_id.clone(),
        })
    }
}

#[derive(Debug, Clone)]
struct ShipOrderCommand {
    order_id: String,
}

impl SagaCommand for ShipOrderCommand {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> futures::future::BoxFuture<'_, Result<(), SagaError>> {
        println!("      [infra] ShipOrderCommand::dispatch — shipping {:?}", self.order_id);
        Box::pin(async move { Ok(()) })
    }
}

#[derive(Debug, Default)]
struct OrderSaga {
    shipped: bool,
}

impl Saga for OrderSaga {
    type SagaId = String;
    type Event = OrderPaid;
    type Command = ShipOrderCommand;

    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, OrderPaid>,
    ) -> Result<SagaHandleResponse<ShipOrderCommand>, SagaError> {
        self.shipped = true;
        Ok(SagaHandleResponse {
            commands: vec![ShipOrderCommand {
                order_id: req.event.order_id.clone(),
            }],
        })
    }

    fn is_complete(&self, _req: SagaIsCompleteRequest) -> Result<SagaIsCompleteResponse, SagaError> {
        Ok(SagaIsCompleteResponse {
            complete: self.shipped,
        })
    }
}

#[derive(Debug, Clone)]
struct SagaEventRequest {
    order_id: String,
}
impl edge_application_base::Request for SagaEventRequest {}

#[derive(Debug, Clone, Copy)]
struct SagaEventResponse {
    commands_dispatched: usize,
}
impl edge_application_base::Response for SagaEventResponse {}

/// Holds its own injected saga behind a lock; genuinely reads `req.ctx` inside `execute()`.
struct SagaEventHandler {
    saga: Arc<Mutex<OrderSaga>>,
}

#[async_trait]
impl Handler for SagaEventHandler {
    type Request = SagaEventRequest;
    type Response = SagaEventResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, SagaEventRequest>,
    ) -> Result<SagaEventResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "saga_event_handler".to_string(),
                message: format!("handling order.paid for {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let event = OrderPaid {
            order_id: req.req.order_id,
        };
        let commands = {
            let mut saga = self.saga.lock();
            saga.handle(SagaHandleRequest { event: &event })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .commands
        };
        for command in &commands {
            command
                .dispatch(SagaCommandDispatchRequest)
                .await
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        }
        Ok(SagaEventResponse {
            commands_dispatched: commands.len(),
        })
    }
}

fn build_handler() -> (SagaEventHandler, Arc<Mutex<OrderSaga>>) {
    let saga = Arc::new(Mutex::new(OrderSaga::default()));
    let handler = SagaEventHandler {
        saga: Arc::clone(&saga),
    };
    (handler, saga)
}

fn run(
    handler: &SagaEventHandler,
    order_id: &str,
) -> Result<SagaEventResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: SagaEventRequest {
            order_id: order_id.to_string(),
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_event_marks_saga_shipped_happy() {
    let (handler, saga) = build_handler();
    let response = run(&handler, "order-1").unwrap();
    assert_eq!(response.commands_dispatched, 1);
    assert!(saga.lock().shipped);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_order_id_still_marks_shipped_error() {
    let (handler, saga) = build_handler();
    let response = run(&handler, "").unwrap();
    assert_eq!(response.commands_dispatched, 1);
    assert!(saga.lock().shipped);
}

/// @covers: Handler::execute
#[test]
fn test_execute_same_saga_twice_stays_shipped_edge() {
    let (handler, saga) = build_handler();
    run(&handler, "order-2").unwrap();
    let second = run(&handler, "order-2").unwrap();
    assert_eq!(second.commands_dispatched, 1);
    assert!(saga.lock().shipped);
}
