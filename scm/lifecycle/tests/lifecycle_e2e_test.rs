//! SAF facade tests — `Lifecycle` trait via `ManagedLifecycle` + `PermissivePolicy`.
// @allow: no_mocks_in_integration — ManagedLifecycle is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{
    Lifecycle, LifecycleError, LifecycleIsInRequest, LifecycleStateRequest,
    LifecycleTransitionRequest, ManagedLifecycle, PermissivePolicy, TransitionAllowedRequest,
    TransitionAllowedResponse, TransitionPolicy, LIFECYCLE_SVC,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Phase {
    Idle,
    Running,
    Paused,
    Done,
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
}

fn permissive(initial: Phase) -> ManagedLifecycle<Phase> {
    ManagedLifecycle::new(initial, Box::new(PermissivePolicy::new()))
}

struct DenyAll;
impl TransitionPolicy for DenyAll {
    type State = Phase;

    fn is_allowed(
        &self,
        _req: TransitionAllowedRequest<Phase>,
    ) -> Result<TransitionAllowedResponse, LifecycleError> {
        Ok(TransitionAllowedResponse { allowed: false })
    }
}

fn strict(initial: Phase) -> ManagedLifecycle<Phase> {
    ManagedLifecycle::new(initial, Box::new(DenyAll))
}

fn state_of(lc: &ManagedLifecycle<Phase>) -> Phase {
    lc.state(LifecycleStateRequest).unwrap().state
}

// ── LIFECYCLE_SVC ─────────────────────────────────────────────────────────────

/// @covers: LIFECYCLE_SVC constant
#[test]
fn test_lifecycle_svc_constant_is_non_empty_happy() {
    assert!(!LIFECYCLE_SVC.is_empty());
}

// ── state ─────────────────────────────────────────────────────────────────────

/// @covers: Lifecycle::state
#[test]
fn test_state_returns_initial_state_happy() {
    assert_eq!(state_of(&permissive(Phase::Idle)), Phase::Idle);
}

/// @covers: Lifecycle::state
#[test]
fn test_state_non_default_initial_reflects_correctly_error() {
    assert_eq!(state_of(&permissive(Phase::Running)), Phase::Running);
}

/// @covers: Lifecycle::state
#[test]
fn test_state_unchanged_when_no_transition_called_edge() {
    let lc = permissive(Phase::Paused);
    let _ = lc.state(LifecycleStateRequest); // read-only access
    assert_eq!(state_of(&lc), Phase::Paused);
}

// ── transition_to — happy ─────────────────────────────────────────────────────

/// @covers: Lifecycle::transition_to
#[test]
fn test_transition_to_allowed_updates_state_happy() {
    rt().block_on(async {
        let lc = permissive(Phase::Idle);
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await
            .expect("allowed");
        assert_eq!(state_of(&lc), Phase::Running);
    });
}

/// @covers: Lifecycle::transition_to
#[test]
fn test_transition_to_chain_updates_each_step_happy() {
    rt().block_on(async {
        let lc = permissive(Phase::Idle);
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await
            .expect("Idle→Running");
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Paused })
            .await
            .expect("Running→Paused");
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Done })
            .await
            .expect("Paused→Done");
        assert_eq!(state_of(&lc), Phase::Done);
    });
}

// ── transition_to — error ─────────────────────────────────────────────────────

/// @covers: Lifecycle::transition_to
#[test]
fn test_transition_to_rejected_returns_invalid_transition_error() {
    rt().block_on(async {
        let lc = strict(Phase::Idle);
        let err = lc
            .transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await
            .expect_err("must fail");
        assert!(matches!(err, LifecycleError::InvalidTransition { .. }));
    });
}

/// @covers: Lifecycle::transition_to
#[test]
fn test_transition_to_rejected_leaves_state_unchanged_error() {
    rt().block_on(async {
        let lc = strict(Phase::Idle);
        let _ = lc
            .transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await;
        assert_eq!(state_of(&lc), Phase::Idle);
    });
}

// ── transition_to — edge ──────────────────────────────────────────────────────

/// @covers: Lifecycle::transition_to
#[test]
fn test_transition_to_same_state_with_permissive_policy_succeeds_edge() {
    rt().block_on(async {
        let lc = permissive(Phase::Running);
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await
            .expect("self-transition allowed");
        assert_eq!(state_of(&lc), Phase::Running);
    });
}

// ── is_in ─────────────────────────────────────────────────────────────────────

/// @covers: Lifecycle::is_in
#[test]
fn test_is_in_returns_true_for_current_state_happy() {
    let lc = permissive(Phase::Idle);
    assert!(lc.is_in(LifecycleIsInRequest { state: Phase::Idle }).unwrap().is_in);
}

/// @covers: Lifecycle::is_in
#[test]
fn test_is_in_returns_false_for_other_state_error() {
    let lc = permissive(Phase::Idle);
    assert!(!lc.is_in(LifecycleIsInRequest { state: Phase::Running }).unwrap().is_in);
}

/// @covers: Lifecycle::is_in
#[test]
fn test_is_in_reflects_state_after_transition_edge() {
    rt().block_on(async {
        let lc = permissive(Phase::Idle);
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Done })
            .await
            .expect("transition");
        assert!(lc.is_in(LifecycleIsInRequest { state: Phase::Done }).unwrap().is_in);
        assert!(!lc.is_in(LifecycleIsInRequest { state: Phase::Idle }).unwrap().is_in);
    });
}
