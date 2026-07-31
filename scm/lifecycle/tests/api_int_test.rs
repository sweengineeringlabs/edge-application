//! Layer-level coverage for `api/lifecycle/dto/*.rs` request/response types.

use edge_application_lifecycle::{
    LifecycleIsInRequest, LifecycleIsInResponse, LifecycleStateRequest, LifecycleStateResponse,
    LifecycleTransitionRequest, TransitionAllowedRequest, TransitionAllowedResponse,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestState {
    A,
    B,
}

/// @covers: LifecycleStateRequest
#[test]
fn test_lifecycle_state_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<LifecycleStateRequest>(), 0);
    let _ = LifecycleStateRequest;
}

/// @covers: LifecycleStateResponse
#[test]
fn test_lifecycle_state_response_holds_state_happy() {
    let r = LifecycleStateResponse { state: TestState::A };
    assert_eq!(r.state, TestState::A);
}

/// @covers: LifecycleIsInRequest
#[test]
fn test_lifecycle_is_in_request_holds_state_happy() {
    let r = LifecycleIsInRequest { state: TestState::B };
    assert_eq!(r.state, TestState::B);
}

/// @covers: LifecycleIsInResponse
#[test]
fn test_lifecycle_is_in_response_holds_flag_happy() {
    let r = LifecycleIsInResponse { is_in: true };
    assert!(r.is_in);
}

/// @covers: LifecycleTransitionRequest
#[test]
fn test_lifecycle_transition_request_holds_target_happy() {
    let r = LifecycleTransitionRequest { target: TestState::B };
    assert_eq!(r.target, TestState::B);
}

/// @covers: TransitionAllowedRequest
#[test]
fn test_transition_allowed_request_holds_from_and_to_happy() {
    let r = TransitionAllowedRequest { from: TestState::A, to: TestState::B };
    assert_eq!(r.from, TestState::A);
    assert_eq!(r.to, TestState::B);
}

/// @covers: TransitionAllowedResponse
#[test]
fn test_transition_allowed_response_holds_flag_error() {
    let r = TransitionAllowedResponse { allowed: false };
    assert!(!r.allowed);
}
