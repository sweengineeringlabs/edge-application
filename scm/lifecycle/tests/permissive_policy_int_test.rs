//! Integration tests for `PermissivePolicy` — covers the types/ file directly.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_lifecycle::{PermissivePolicy, TransitionAllowedRequest, TransitionPolicy};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum S {
    A,
    B,
    C,
}

fn allowed(p: &PermissivePolicy<S>, from: S, to: S) -> bool {
    p.is_allowed(TransitionAllowedRequest { from, to }).unwrap().allowed
}

/// @covers: PermissivePolicy — allows any forward transition
#[test]
fn test_is_allowed_forward_transition_returns_true_happy() {
    let p = PermissivePolicy::new();
    assert!(allowed(&p, S::A, S::B));
    assert!(allowed(&p, S::B, S::C));
}

/// @covers: PermissivePolicy — allows self-transition (same state → same state)
#[test]
fn test_is_allowed_self_transition_returns_true_error() {
    let p = PermissivePolicy::new();
    assert!(allowed(&p, S::A, S::A));
}

/// @covers: PermissivePolicy — allows backward transition (no restrictions)
#[test]
fn test_is_allowed_backward_transition_returns_true_edge() {
    let p = PermissivePolicy::new();
    assert!(allowed(&p, S::C, S::A));
}
