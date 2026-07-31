//! Runnable example: two `Handler`s sharing one constructor-injected `Policy<Input>`, with the
//! per-call value to evaluate carried through `Self::Request` — the generic-per-type port
//! wiring pattern `HandlerContext` structurally can't hold. See issue #149.
//!
//! `Policy::name`/`evaluate` both take `&self` and are fully sync — the simplest port in this
//! series, no lock needed, straightforward `Arc<dyn Policy<...>>` injection like
//! `repository`/`event`/`snapshot`.
//!
//! `WithdrawalPolicyHandler::execute` genuinely reads `HandlerContext` for logging. Both
//! handlers here share the same injected `Arc<dyn Policy<Input = u64>>` — a `CompositePolicy`
//! ANDing a minimum and a maximum bound.
//!
//! Run with: `cargo run -p edge-application-policy-handler-example`

mod api;
mod core;

use std::sync::Arc;

use edge_application_command::DirectCommandBus;
use edge_application_handler::{ExecutionRequest as HandlerExecutionRequest, Handler, HandlerContext};
use edge_application_observer::StdObserveFactory;
use edge_application_policy::{CompositePolicy, Policy};
use edge_security_runtime::SecurityContext;

use crate::api::{MaxAmountPolicy, MinAmountPolicy, PolicyIdentityRequest, WithdrawalPolicyRequest};
use crate::core::{PolicyIdentityHandler, WithdrawalPolicyHandler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    println!("=== Handler + injected Policy<Input> — constructor injection, Self::Request carries the value ===\n");

    let policy: Arc<dyn Policy<Input = u64>> = Arc::new(
        CompositePolicy::new()
            .with(Box::new(MinAmountPolicy(10)))
            .with(Box::new(MaxAmountPolicy(1000))),
    );
    let withdrawal_handler = WithdrawalPolicyHandler {
        policy: Arc::clone(&policy),
    };
    let name_handler = PolicyIdentityHandler {
        policy: Arc::clone(&policy),
    };

    let security = SecurityContext::unauthenticated();
    let bus = DirectCommandBus;
    let observer = StdObserveFactory::noop_observer_context();
    let ctx = HandlerContext {
        security: &security,
        commands: &bus,
        observer: observer.as_ref(),
    };

    println!("[0] name_handler.execute(PolicyIdentityRequest)");
    let name_resp = name_handler
        .execute(HandlerExecutionRequest {
            req: PolicyIdentityRequest,
            ctx: &ctx,
        })
        .await
        .expect("name should succeed");
    println!("[3] PolicyIdentityHandler reports name = {:?}\n", name_resp.name);

    println!("[0] withdrawal_handler.execute(WithdrawalPolicyRequest {{ amount: 500 }})");
    let allowed = withdrawal_handler
        .execute(HandlerExecutionRequest {
            req: WithdrawalPolicyRequest { amount: 500 },
            ctx: &ctx,
        })
        .await
        .expect("500 should be within bounds");
    println!("[3] WithdrawalPolicyHandler reports allowed = {}\n", allowed.allowed);

    println!("[0] withdrawal_handler.execute(WithdrawalPolicyRequest {{ amount: 5 }}) — below minimum");
    let rejected = withdrawal_handler
        .execute(HandlerExecutionRequest {
            req: WithdrawalPolicyRequest { amount: 5 },
            ctx: &ctx,
        })
        .await;
    match rejected {
        Ok(_) => println!("[3] unexpectedly succeeded\n"),
        Err(e) => println!("[3] WithdrawalPolicyHandler returned an error, as expected: {e}\n"),
    }

    println!("Conclusion: WithdrawalPolicyHandler used ctx.observer for real per-request logging, but");
    println!("both handlers reached the actual policy only through their own, independently");
    println!("injected Policy<u64> — never through HandlerContext, which structurally cannot hold a");
    println!("generic-per-type port. The composite policy's own violation propagated cleanly as a");
    println!("real HandlerError.");
}
