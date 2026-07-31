//! SAF facade tests — `ManagedLifecycle` construction via `PermissivePolicy`/custom policies.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{
    Lifecycle, LifecycleError, LifecycleStateRequest, LifecycleTransitionRequest, ManagedLifecycle,
    PermissivePolicy, TransitionAllowedRequest, TransitionAllowedResponse, TransitionPolicy,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum S {
    A,
    B,
    C,
}

struct DenyAll;
impl TransitionPolicy for DenyAll {
    type State = S;

    fn is_allowed(
        &self,
        _req: TransitionAllowedRequest<S>,
    ) -> Result<TransitionAllowedResponse, LifecycleError> {
        Ok(TransitionAllowedResponse { allowed: false })
    }
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
}

fn state_of<L: Lifecycle>(lc: &L) -> L::State {
    lc.state(LifecycleStateRequest).unwrap().state
}

fn permissive(initial: S) -> ManagedLifecycle<S> {
    ManagedLifecycle::new(initial, Box::new(PermissivePolicy::new()))
}

// ── managed ───────────────────────────────────────────────────────────────────

/// @covers: ManagedLifecycle::new
#[test]
fn test_managed_starts_in_initial_state_happy() {
    let lc = permissive(S::A);
    assert_eq!(state_of(&lc), S::A);
}

/// @covers: ManagedLifecycle::new
#[test]
fn test_managed_with_deny_policy_rejects_transition_error() {
    rt().block_on(async {
        let lc = ManagedLifecycle::new(S::A, Box::new(DenyAll));
        let err = lc
            .transition_to(LifecycleTransitionRequest { target: S::B })
            .await
            .expect_err("must fail");
        assert!(matches!(err, LifecycleError::InvalidTransition { .. }));
    });
}

/// @covers: ManagedLifecycle::new
#[test]
fn test_managed_with_permissive_policy_allows_chain_edge() {
    rt().block_on(async {
        let lc = permissive(S::A);
        lc.transition_to(LifecycleTransitionRequest { target: S::B })
            .await
            .expect("A→B");
        lc.transition_to(LifecycleTransitionRequest { target: S::C })
            .await
            .expect("B→C");
        assert_eq!(state_of(&lc), S::C);
    });
}

// ── permissive ────────────────────────────────────────────────────────────────

/// @covers: PermissivePolicy::new
#[test]
fn test_permissive_starts_in_initial_state_happy() {
    let lc = permissive(S::A);
    assert_eq!(state_of(&lc), S::A);
}

/// @covers: PermissivePolicy::new
#[test]
fn test_permissive_does_not_reject_any_transition_error() {
    rt().block_on(async {
        // "error" scenario: verify there is no situation where permissive rejects
        let lc = permissive(S::C);
        lc.transition_to(LifecycleTransitionRequest { target: S::A })
            .await
            .expect("backward transition allowed");
        assert_eq!(state_of(&lc), S::A);
    });
}

/// @covers: PermissivePolicy::new
#[test]
fn test_permissive_allows_self_transition_edge() {
    rt().block_on(async {
        let lc = permissive(S::B);
        lc.transition_to(LifecycleTransitionRequest { target: S::B })
            .await
            .expect("self-transition allowed");
        assert_eq!(state_of(&lc), S::B);
    });
}
