//! Integration test: `PolicyIdentityHandler::execute()` reads through a real,
//! constructor-injected `Policy` — proving the same handler shape works whether the injected
//! policy is a single rule or a `CompositePolicy` combining several, since both satisfy the
//! same `Policy<Input = u64>` trait object.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use async_trait::async_trait;
use edge_application_command::DirectCommandBus;
use edge_application_handler::{
    ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext, HandlerError,
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

#[derive(Debug, Clone, Copy, Default)]
struct PolicyIdentityRequest;
impl edge_application_base::Request for PolicyIdentityRequest {}

#[derive(Debug, Clone)]
struct PolicyIdentityResponse {
    name: String,
}
impl edge_application_base::Response for PolicyIdentityResponse {}

/// Holds its own injected `Policy` — the per-call data (none, here) would arrive through
/// `Self::Request` for a richer example; this handler's whole point is the injected port.
struct PolicyIdentityHandler {
    policy: Arc<dyn Policy<Input = u64>>,
}

#[async_trait]
impl Handler for PolicyIdentityHandler {
    type Request = PolicyIdentityRequest;
    type Response = PolicyIdentityResponse;

    async fn execute(
        &self,
        _req: HandlerExecutionRequest<'_, PolicyIdentityRequest>,
    ) -> Result<PolicyIdentityResponse, HandlerError> {
        let name = self
            .policy
            .name(PolicyNameRequest)
            .map_err(|e| HandlerError::ExecutionFailed(e.to_string()))?
            .name;
        Ok(PolicyIdentityResponse {
            name: name.to_string(),
        })
    }
}

fn run(handler: &PolicyIdentityHandler) -> Result<PolicyIdentityResponse, HandlerError> {
    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    block_on(handler.execute(HandlerExecutionRequest {
        req: PolicyIdentityRequest,
        ctx: &ctx,
    }))
}

/// @covers: Handler::execute
#[test]
fn test_execute_composite_policy_returns_composite_name_happy() {
    let policy: Arc<dyn Policy<Input = u64>> =
        Arc::new(CompositePolicy::new().with(Box::new(MinAmountPolicy(10))));
    let handler = PolicyIdentityHandler { policy };

    let response = run(&handler).unwrap();
    assert_eq!(response.name, "composite");
}

/// @covers: Handler::execute
#[test]
fn test_execute_single_policy_returns_its_own_name_error() {
    let policy: Arc<dyn Policy<Input = u64>> = Arc::new(MinAmountPolicy(10));
    let handler = PolicyIdentityHandler { policy };

    let response = run(&handler).unwrap();
    assert_eq!(response.name, "min-amount");
}

/// @covers: Handler::execute
#[test]
fn test_execute_repeated_calls_are_consistent_edge() {
    let policy: Arc<dyn Policy<Input = u64>> = Arc::new(MinAmountPolicy(10));
    let handler = PolicyIdentityHandler { policy };

    let first = run(&handler).unwrap();
    let second = run(&handler).unwrap();
    assert_eq!(first.name, second.name);
}
