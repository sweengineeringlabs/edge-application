//! Integration tests for `TransitionPolicy` — covers the traits/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{TransitionAllowedRequest, TransitionPolicy};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Gate {
    Open,
    Closed,
    Locked,
}

/// A custom policy: Locked is terminal — nothing transitions out of it.
struct LockFinalPolicy;

impl TransitionPolicy for LockFinalPolicy {
    type State = Gate;

    fn is_allowed(
        &self,
        req: TransitionAllowedRequest<Gate>,
    ) -> Result<edge_application_lifecycle::TransitionAllowedResponse, edge_application_lifecycle::LifecycleError> {
        Ok(edge_application_lifecycle::TransitionAllowedResponse {
            allowed: req.to != Gate::Locked || req.from == Gate::Open,
        })
    }
}

fn allowed(p: &LockFinalPolicy, from: Gate, to: Gate) -> bool {
    p.is_allowed(TransitionAllowedRequest { from, to }).unwrap().allowed
}

/// @covers: TransitionPolicy::is_allowed — custom policy allows a permitted transition
#[test]
fn test_is_allowed_permitted_transition_returns_true_happy() {
    let p = LockFinalPolicy;
    assert!(allowed(&p, Gate::Open, Gate::Closed));
    assert!(allowed(&p, Gate::Closed, Gate::Open));
}

/// @covers: TransitionPolicy::is_allowed — custom policy rejects a forbidden transition
#[test]
fn test_is_allowed_forbidden_transition_returns_false_error() {
    let p = LockFinalPolicy;
    assert!(!allowed(&p, Gate::Closed, Gate::Locked));
}

/// @covers: TransitionPolicy::is_allowed — self-transition semantics depend on policy
#[test]
fn test_is_allowed_self_transition_depends_on_policy_edge() {
    let p = LockFinalPolicy;
    // Open→Open: `to` is Open, not Locked, so allowed
    assert!(allowed(&p, Gate::Open, Gate::Open));
    // Locked→Locked: `to` is Locked and `from` is not Open, so forbidden
    assert!(!allowed(&p, Gate::Locked, Gate::Locked));
}
