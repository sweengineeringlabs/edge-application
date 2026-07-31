//! SAF facade tests — `EventBus` trait via `NoopEventBus` and `InProcessEventBus`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use edge_application_event::{
    DomainEvent, EventBus, EventBusPublishRequest, EventBusSubscribeRequest, EventError,
    EventSource, EventSourceRecvNextRequest, EventTypeRequest, InProcessEventBus, NoopEventBus,
};

struct Evt;
impl DomainEvent for Evt {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "evt" })
    }
}

/// @covers: NoopEventBus::publish — always Ok
#[test]
fn test_publish_noop_returns_ok_happy() {
    let result = futures::executor::block_on(
        NoopEventBus.publish(EventBusPublishRequest { event: Arc::new(Evt) }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventBus::subscribe — receiver is immediately closed
#[test]
fn test_subscribe_noop_receiver_unavailable_error() {
    let mut rx = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}

/// @covers: InProcessEventBus::publish — succeeds with subscriber
#[test]
fn test_in_process_bus_publish_with_subscriber_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(8);
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        bus.publish(EventBusPublishRequest { event: Arc::new(Evt) }).await.expect("publish");
        let e = rx.recv_next(EventSourceRecvNextRequest).await.expect("recv").event;
        assert_eq!(e.event_type(EventTypeRequest).unwrap().event_type, "evt");
    });
}

/// @covers: InProcessEventBus::publish — no subscribers still returns Ok
#[test]
fn test_in_process_bus_publish_no_subscribers_returns_ok_edge() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(edge_application_event::EventBusConfig::default().capacity);
        assert!(bus.publish(EventBusPublishRequest { event: Arc::new(Evt) }).await.is_ok());
    });
}

/// @covers: NoopEventBus::publish — noop bus never errors even for repeated publishes
#[test]
fn test_publish_noop_repeated_publishes_all_ok_error() {
    for _ in 0..3 {
        let result = futures::executor::block_on(
            NoopEventBus.publish(EventBusPublishRequest { event: Arc::new(Evt) }),
        );
        assert_eq!(result, Ok(()));
    }
}

/// @covers: NoopEventBus::subscribe — subscribe returns a receiver
#[test]
fn test_subscribe_noop_returns_receiver_happy() {
    // subscribe must not panic; receiver is a `Box<dyn EventSource>` so it is
    // never zero-sized regardless of which EventBus impl produced it.
    let _rx = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    assert!(std::mem::size_of_val(&_rx) > 0);
}

/// @covers: NoopEventBus::subscribe — multiple subscribe calls each get a closed receiver
#[test]
fn test_subscribe_noop_multiple_calls_all_closed_edge() {
    for _ in 0..3 {
        let mut rx = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(matches!(
            futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest)),
            Err(EventError::Unavailable(_))
        ));
    }
}
