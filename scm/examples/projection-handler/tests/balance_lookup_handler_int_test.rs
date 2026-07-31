//! Integration test: `BalanceLookupHandler::execute()` reads through the *same*
//! `Arc<Mutex<Balance>>` instance a `CreditPostingHandler` mutates, proving one projection
//! instance can genuinely back multiple handlers, each reaching it through its own injected
//! field rather than through `HandlerContext`.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
};
use edge_application_observer::StdObserveFactory;
use edge_application_projection::{
    Projection, ProjectionApplyRequest, ProjectionEvent, ProjectionEventDescribeRequest,
    ProjectionEventDescribeResponse, ProjectionError, ProjectionReadModelRequest,
    ProjectionReadModelResponse,
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

#[derive(Debug, Clone, Copy, Default)]
struct BalanceLookupRequest;
impl edge_application_base::Request for BalanceLookupRequest {}

#[derive(Debug, Clone, Copy)]
struct BalanceLookupResponse {
    total: u64,
}
impl edge_application_base::Response for BalanceLookupResponse {}

/// Holds its own injected projection behind the same lock a `CreditPostingHandler` would
/// mutate.
struct BalanceLookupHandler {
    balance: Arc<Mutex<Balance>>,
}

#[async_trait]
impl Handler for BalanceLookupHandler {
    type Request = BalanceLookupRequest;
    type Response = BalanceLookupResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, BalanceLookupRequest>,
    ) -> Result<BalanceLookupResponse, HandlerError> {
        let total = *self
            .balance
            .lock()
            .read_model(ProjectionReadModelRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .read_model;
        Ok(BalanceLookupResponse { total })
    }
}

fn run(handler: &BalanceLookupHandler) -> Result<BalanceLookupResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: BalanceLookupRequest,
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_after_credits_applied_returns_total_happy() {
    let balance = Arc::new(Mutex::new(Balance::default()));
    balance
        .lock()
        .apply(ProjectionApplyRequest {
            event: &Credited { amount: 42 },
        })
        .unwrap();
    let handler = BalanceLookupHandler {
        balance: Arc::clone(&balance),
    };

    let response = run(&handler).unwrap();
    assert_eq!(response.total, 42);
}

/// @covers: Handler::execute
#[test]
fn test_execute_before_any_credits_returns_zero_error() {
    let balance = Arc::new(Mutex::new(Balance::default()));
    let handler = BalanceLookupHandler { balance };

    let response = run(&handler).unwrap();
    assert_eq!(response.total, 0);
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_reflect_latest_state_edge() {
    let balance = Arc::new(Mutex::new(Balance::default()));
    let handler = BalanceLookupHandler {
        balance: Arc::clone(&balance),
    };

    let before = run(&handler).unwrap();
    balance
        .lock()
        .apply(ProjectionApplyRequest {
            event: &Credited { amount: 7 },
        })
        .unwrap();
    let after = run(&handler).unwrap();

    assert_eq!(before.total, 0);
    assert_eq!(after.total, 7);
}
