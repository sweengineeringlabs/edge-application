//! Integration tests for [`NoopSaga`].
// @allow: no_mocks_in_integration
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_saga::{NoopSaga, Saga, SagaHandleRequest, SagaIsCompleteRequest};

/// @covers: is_complete
#[test]
fn test_is_complete_default_noop_saga_returns_false_happy() {
    let saga = NoopSaga::default();
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}

/// @covers: handle
#[test]
fn test_handle_noop_saga_returns_empty_commands_error() {
    // NoopSaga::handle always returns empty — it is never the "happy" path for work
    use edge_application_saga::NoopSagaEvent;
    let mut saga = NoopSaga::default();
    let cmds = saga
        .handle(SagaHandleRequest {
            event: &NoopSagaEvent,
        })
        .unwrap()
        .commands;
    assert!(cmds.is_empty());
}

/// @covers: is_complete
#[test]
fn test_is_complete_noop_saga_never_completes_edge() {
    use edge_application_saga::NoopSagaEvent;
    let mut saga = NoopSaga::default();
    saga.handle(SagaHandleRequest {
        event: &NoopSagaEvent,
    })
    .unwrap();
    saga.handle(SagaHandleRequest {
        event: &NoopSagaEvent,
    })
    .unwrap();
    // NoopSaga cannot reach completion — complete starts false and handle() never sets it
    assert!(!saga.is_complete(SagaIsCompleteRequest).unwrap().complete);
}
