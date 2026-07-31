//! Integration test: `CreditPostingHandler::execute()` genuinely reads `HandlerContext` for real
//! observability and, as part of the same call, mutates a real, constructor-injected
//! `Projection` through `Arc<Mutex<P>>` — `Handler` -> `Projection`, connected and working, as
//! one observable call chain. Assertions verify the read model's actual state changed, and that
//! `try_drain`'s own empty-batch validation propagates as a genuine `HandlerError`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_projection::{
    Projection, ProjectionApplyRequest, ProjectionEvent, ProjectionEventDescribeRequest,
    ProjectionEventDescribeResponse, ProjectionError, ProjectionReadModelRequest,
    ProjectionReadModelResponse, TryDrainRequest,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;
use parking_lot::Mutex;

#[derive(Debug, Clone, Copy)]
struct Credited {
    amount: u64,
}

impl ProjectionEvent for Credited {
    fn describe(
        &self,
        _req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError> {
        Ok(ProjectionEventDescribeResponse {
            event_type: "credited".to_string(),
            aggregate_id: "balance".to_string(),
        })
    }
}

#[derive(Debug, Default)]
struct Balance {
    total: u64,
}

impl Projection for Balance {
    type Event = Credited;
    type ReadModel = u64;

    fn apply(&mut self, req: ProjectionApplyRequest<'_, Credited>) -> Result<(), ProjectionError> {
        self.total += req.event.amount;
        Ok(())
    }

    fn read_model(
        &self,
        _req: ProjectionReadModelRequest,
    ) -> Result<ProjectionReadModelResponse<'_, u64>, ProjectionError> {
        Ok(ProjectionReadModelResponse {
            read_model: &self.total,
        })
    }
}

#[derive(Debug, Clone)]
struct CreditPostingRequest {
    amounts: Vec<u64>,
}
impl edge_application_base::Request for CreditPostingRequest {}

#[derive(Debug, Clone, Copy)]
struct CreditPostingResponse {
    applied: usize,
}
impl edge_application_base::Response for CreditPostingResponse {}

/// Holds its own injected projection behind a lock; genuinely reads `req.ctx`.
struct CreditPostingHandler {
    balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for CreditPostingHandler {
    type Request = CreditPostingRequest;
    type Response = CreditPostingResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, CreditPostingRequest>,
    ) -> Result<CreditPostingResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "credit_posting_handler".to_string(),
                message: format!("applying {} credit event(s)", req.req.amounts.len()),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        let events: Vec<Credited> = req
            .req
            .amounts
            .iter()
            .map(|&amount| Credited { amount })
            .collect();
        let count = {
            let mut balance = self.balance.lock();
            balance
                .try_drain(TryDrainRequest { events: &events })
                .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
                .count
        };
        Ok(CreditPostingResponse { applied: count })
    }
}

fn build_handler() -> (CreditPostingHandler, Arc<Mutex<Balance>>) {
    let balance = Arc::new(Mutex::new(Balance::default()));
    let handler = CreditPostingHandler {
        balance: Arc::clone(&balance),
    };
    (handler, balance)
}

fn run(
    handler: &CreditPostingHandler,
    amounts: Vec<u64>,
) -> Result<CreditPostingResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: CreditPostingRequest { amounts },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_credits_update_read_model_happy() {
    let (handler, balance) = build_handler();
    let response = run(&handler, vec![10, 20, 5]).unwrap();
    assert_eq!(response.applied, 3);
    assert_eq!(balance.lock().total, 35);
}

/// @covers: Handler::execute
#[test]
fn test_execute_empty_batch_returns_execution_failed_error() {
    let (handler, _balance) = build_handler();
    let err = run(&handler, vec![]).unwrap_err();
    assert!(matches!(err, HandlerError::ExecutionFailed(_)));
}

/// @covers: Handler::execute
#[test]
fn test_execute_multiple_batches_accumulate_edge() {
    let (handler, balance) = build_handler();
    run(&handler, vec![10]).unwrap();
    run(&handler, vec![5, 5]).unwrap();
    assert_eq!(balance.lock().total, 20);
}
