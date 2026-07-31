//! SAF facade tests — `TransitionPolicy` service (`transition_policy_svc`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{
    LifecycleError, TransitionAllowedRequest, TransitionAllowedResponse, TransitionPolicy,
    TRANSITION_POLICY_SVC,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum State {
    Off,
    On,
    Fault,
}

/// Policy: On→Fault and Fault→Off are the only allowed transitions.
struct RecoveryOnlyPolicy;

impl TransitionPolicy for RecoveryOnlyPolicy {
    type State = State;

    fn is_allowed(
        &self,
        req: TransitionAllowedRequest<State>,
    ) -> Result<TransitionAllowedResponse, LifecycleError> {
        Ok(TransitionAllowedResponse {
            allowed: matches!(
                (req.from, req.to),
                (State::On, State::Fault) | (State::Fault, State::Off)
            ),
        })
    }
}

fn allowed(p: &RecoveryOnlyPolicy, from: State, to: State) -> bool {
    p.is_allowed(TransitionAllowedRequest { from, to }).unwrap().allowed
}

// ── TRANSITION_POLICY_SVC ─────────────────────────────────────────────────────

/// @covers: TRANSITION_POLICY_SVC — constant is non-empty
#[test]
fn test_transition_policy_svc_constant_is_non_empty_happy() {
    assert!(!TRANSITION_POLICY_SVC.is_empty());
}

// ── TransitionPolicy::is_allowed ──────────────────────────────────────────────

/// @covers: TransitionPolicy::is_allowed — permitted transition returns true
#[test]
fn test_is_allowed_permitted_transition_returns_true_happy() {
    let p = RecoveryOnlyPolicy;
    assert!(allowed(&p, State::On, State::Fault));
    assert!(allowed(&p, State::Fault, State::Off));
}

/// @covers: TransitionPolicy::is_allowed — forbidden transition returns false
#[test]
fn test_is_allowed_forbidden_transition_returns_false_error() {
    let p = RecoveryOnlyPolicy;
    assert!(!allowed(&p, State::Off, State::On));
    assert!(!allowed(&p, State::On, State::Off));
}

/// @covers: TransitionPolicy::is_allowed — self-transition obeys policy
#[test]
fn test_is_allowed_self_transition_obeys_policy_edge() {
    let p = RecoveryOnlyPolicy;
    assert!(!allowed(&p, State::On, State::On));
    assert!(!allowed(&p, State::Off, State::Off));
}
