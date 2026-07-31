//! Integration tests for `ManagedLifecycle` — covers the types/ file directly.
// @allow: no_mocks_in_integration — ManagedLifecycle is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{
    Lifecycle, LifecycleStateRequest, LifecycleTransitionRequest, ManagedLifecycle,
    PermissivePolicy,
};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Phase {
    Idle,
    Running,
}

fn rt() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_current_thread()
        .build()
        .expect("rt")
}

fn state_of(lc: &ManagedLifecycle<Phase>) -> Phase {
    lc.state(LifecycleStateRequest).unwrap().state
}

/// @covers: ManagedLifecycle::new — starts in the given initial state
#[test]
fn test_new_starts_in_initial_state_happy() {
    let lc = ManagedLifecycle::new(Phase::Idle, Box::new(PermissivePolicy::new()));
    assert_eq!(state_of(&lc), Phase::Idle);
}

/// @covers: ManagedLifecycle — non-default initial state is preserved
#[test]
fn test_new_non_default_initial_state_preserved_error() {
    let lc = ManagedLifecycle::new(Phase::Running, Box::new(PermissivePolicy::new()));
    assert_eq!(state_of(&lc), Phase::Running);
}

/// @covers: ManagedLifecycle — state after transition is the target state
#[test]
fn test_state_after_transition_is_target_edge() {
    rt().block_on(async {
        let lc = ManagedLifecycle::new(Phase::Idle, Box::new(PermissivePolicy::new()));
        lc.transition_to(LifecycleTransitionRequest { target: Phase::Running })
            .await
            .expect("transition");
        assert_eq!(state_of(&lc), Phase::Running);
    });
}
