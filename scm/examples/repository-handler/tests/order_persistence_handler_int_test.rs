//! Integration test: `OrderPersistenceHandler::execute()` genuinely reads `HandlerContext` for real
//! observability and, as part of the same call, persists through a real, constructor-injected
//! `Repository` — `Handler` -> `Repository`, connected and working, as one observable call
//! chain. Assertions verify the write landed in the shared repository, not just that
//! `execute()` returned `Ok`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_repository::{
    MemoryRepository, Repository, RepositoryIdRequest, RepositorySaveRequest,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Order {
    item: String,
    quantity: u32,
}

#[derive(Debug, Clone)]
struct OrderPersistenceRequest {
    order_id: String,
    item: String,
    quantity: u32,
}
impl edge_application_base::Request for OrderPersistenceRequest {}

#[derive(Debug, Clone, Copy)]
struct OrderPersistenceResponse {
    saved: bool,
}
impl edge_application_base::Response for OrderPersistenceResponse {}

/// Holds its own injected `Repository`; genuinely reads `req.ctx` inside `execute()`.
struct OrderPersistenceHandler {
    repository: Arc<dyn Repository<Entity = Order, Id = String>>,
}

#[async_trait]
impl Handler for OrderPersistenceHandler {
    type Request = OrderPersistenceRequest;
    type Response = OrderPersistenceResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, OrderPersistenceRequest>,
    ) -> Result<OrderPersistenceResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "order_persistence_handler".to_string(),
                message: format!("saving order {:?}", req.req.order_id),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        self.repository
            .save(RepositorySaveRequest {
                id: req.req.order_id,
                entity: Order {
                    item: req.req.item,
                    quantity: req.req.quantity,
                },
            })
            .await
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(OrderPersistenceResponse { saved: true })
    }
}

fn build_handler() -> (
    OrderPersistenceHandler,
    Arc<dyn Repository<Entity = Order, Id = String>>,
) {
    let repository: Arc<dyn Repository<Entity = Order, Id = String>> =
        Arc::new(MemoryRepository::<Order, String>::new());
    let handler = OrderPersistenceHandler {
        repository: Arc::clone(&repository),
    };
    (handler, repository)
}

fn run(
    handler: &OrderPersistenceHandler,
    order_id: &str,
    item: &str,
    quantity: u32,
) -> Result<OrderPersistenceResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: OrderPersistenceRequest {
            order_id: order_id.to_string(),
            item: item.to_string(),
            quantity,
        },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_new_order_persists_in_repository_happy() {
    let (handler, repository) = build_handler();
    let response = run(&handler, "order-1", "widget", 3).unwrap();
    assert!(response.saved);

    let found = block_on(repository.find(RepositoryIdRequest { id: &"order-1".to_string() }))
        .unwrap()
        .entity;
    assert_eq!(
        found,
        Some(Order {
            item: "widget".to_string(),
            quantity: 3,
        })
    );
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_order_id_still_persists_error() {
    let (handler, repository) = build_handler();
    let response = run(&handler, "", "mystery-item", 1).unwrap();
    assert!(response.saved);

    let found = block_on(repository.find(RepositoryIdRequest { id: &String::new() }))
        .unwrap()
        .entity;
    assert!(found.is_some());
}

/// @covers: Handler::execute
#[test]
fn test_execute_same_id_twice_last_write_wins_edge() {
    let (handler, repository) = build_handler();
    run(&handler, "order-2", "first-item", 1).unwrap();
    run(&handler, "order-2", "second-item", 9).unwrap();

    let found = block_on(repository.find(RepositoryIdRequest { id: &"order-2".to_string() }))
        .unwrap()
        .entity;
    assert_eq!(
        found,
        Some(Order {
            item: "second-item".to_string(),
            quantity: 9,
        })
    );
}
