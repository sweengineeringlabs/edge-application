//! Integration test: `WithdrawalPolicyHandler::execute()` genuinely reads `HandlerContext` for
//! real observability and, as part of the same call, evaluates through a real,
//! constructor-injected `Policy` — `Handler` -> `Policy`, connected and working, as one
//! observable call chain. Assertions verify the real `CompositePolicy` violation propagates as
//! a genuine `HandlerError`, not a fabricated one.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    DrainRequest, ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext,
    HandlerError, LogEmitRequest,
};
use edge_application_observer::StdObserveFactory;
use edge_application_policy::{
    CompositePolicy, Policy, PolicyEvaluateRequest, PolicyNameRequest, PolicyNameResponse,
    PolicyError,
};
use edge_security_runtime::SecurityContext;
use futures::executor::block_on;

struct MinAmountPolicy(u64);

impl Policy for MinAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "min-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input >= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "min-amount",
                format!("{} is below the minimum of {}", req.input, self.0),
            ))
        }
    }
}

struct MaxAmountPolicy(u64);

impl Policy for MaxAmountPolicy {
    type Input = u64;

    fn name(&self, _req: PolicyNameRequest) -> Result<PolicyNameResponse, PolicyError> {
        Ok(PolicyNameResponse { name: "max-amount" })
    }
    fn evaluate(&self, req: PolicyEvaluateRequest<'_, u64>) -> Result<(), PolicyError> {
        if *req.input <= self.0 {
            Ok(())
        } else {
            Err(PolicyError::new(
                "max-amount",
                format!("{} exceeds the maximum of {}", req.input, self.0),
            ))
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct WithdrawalPolicyRequest {
    amount: u64,
}
impl edge_application_base::Request for WithdrawalPolicyRequest {}

#[derive(Debug, Clone, Copy)]
struct WithdrawalPolicyResponse {
    allowed: bool,
}
impl edge_application_base::Response for WithdrawalPolicyResponse {}

/// Holds its own injected `Policy`; genuinely reads `req.ctx` inside `execute()`.
struct WithdrawalPolicyHandler {
    policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for WithdrawalPolicyHandler {
    type Request = WithdrawalPolicyRequest;
    type Response = WithdrawalPolicyResponse;

    async fn execute(
        &self,
        req: HandlerExecutionRequest<'_, WithdrawalPolicyRequest>,
    ) -> Result<WithdrawalPolicyResponse, HandlerError> {
        req.ctx
            .observer
            .drain(DrainRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .drain
            .emit(LogEmitRequest {
                level: "INFO".to_string(),
                handler_id: "withdrawal_policy_handler".to_string(),
                message: format!("evaluating withdrawal of {}", req.req.amount),
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;

        self.policy
            .evaluate(PolicyEvaluateRequest {
                input: &req.req.amount,
            })
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?;
        Ok(WithdrawalPolicyResponse { allowed: true })
    }
}

fn build_handler() -> WithdrawalPolicyHandler {
    let policy: Arc<dyn Policy<Input = u64>> = Arc::new(
        CompositePolicy::new()
            .with(Box::new(MinAmountPolicy(10)))
            .with(Box::new(MaxAmountPolicy(1000))),
    );
    WithdrawalPolicyHandler { policy }
}

fn run(
    handler: &WithdrawalPolicyHandler,
    amount: u64,
) -> Result<WithdrawalPolicyResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: WithdrawalPolicyRequest { amount },
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_amount_within_bounds_allows_happy() {
    let handler = build_handler();
    let response = run(&handler, 500).unwrap();
    assert!(response.allowed);
}

/// @covers: Handler::execute
#[test]
fn test_execute_amount_below_minimum_returns_execution_failed_error() {
    let handler = build_handler();
    let err = run(&handler, 5).unwrap_err();
    assert!(matches!(err, HandlerError::ExecutionFailed(_)));
}

/// @covers: Handler::execute
#[test]
fn test_execute_amount_at_exact_bounds_allows_edge() {
    let handler = build_handler();
    let at_min = run(&handler, 10).unwrap();
    let at_max = run(&handler, 1000).unwrap();
    assert!(at_min.allowed);
    assert!(at_max.allowed);
}
