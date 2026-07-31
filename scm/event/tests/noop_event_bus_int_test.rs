//! Integration tests for `NoopEventBus`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use edge_application_event::{
    DomainEvent, EventBus, EventBusPublishRequest, EventBusSubscribeRequest, EventError,
    EventSource, EventSourceRecvNextRequest, NoopEventBus,
};

struct SignalEvt;
impl DomainEvent for SignalEvt {}

/// @covers: NoopEventBus::publish — always returns Ok
#[test]
fn test_noop_event_bus_publish_returns_ok_happy() {
    let result = futures::executor::block_on(
        NoopEventBus.publish(EventBusPublishRequest { event: Arc::new(SignalEvt) }),
    );
    assert_eq!(result, Ok(()), "noop bus publish must always return Ok(())");
}

/// @covers: NoopEventBus::publish — repeated publishes never error
#[test]
fn test_noop_event_bus_publish_repeated_never_errors_error() {
    for _ in 0..5 {
        let result = futures::executor::block_on(
            NoopEventBus.publish(EventBusPublishRequest { event: Arc::new(SignalEvt) }),
        );
        assert_eq!(result, Ok(()));
    }
}

/// @covers: NoopEventBus::subscribe — subscribe returns immediately-closed receiver
#[test]
fn test_noop_event_bus_subscribe_returns_closed_receiver_edge() {
    let mut rx = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}
