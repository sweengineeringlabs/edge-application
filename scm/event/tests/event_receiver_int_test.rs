//! Integration tests for `EventSource::recv_next` via `EventBusSubscribeResponse::receiver`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use edge_application_event::{
    DomainEvent, EventBus, EventBusPublishRequest, EventBusSubscribeRequest, EventError,
    EventSource, EventSourceRecvNextRequest, EventTypeRequest, InProcessEventBus,
};

struct PingEvt;
impl DomainEvent for PingEvt {
    fn event_type(&self, _req: EventTypeRequest) -> Result<edge_application_event::EventTypeResponse<'_>, EventError> {
        Ok(edge_application_event::EventTypeResponse { event_type: "ping" })
    }
}

/// @covers: EventSource::recv_next — receives event published to bus
#[test]
fn test_event_receiver_recv_returns_published_event_happy() {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(4);
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        bus.publish(EventBusPublishRequest { event: Arc::new(PingEvt) }).await.expect("publish");
        let event = rx.recv_next(EventSourceRecvNextRequest).await.expect("recv").event;
        assert_eq!(event.event_type(EventTypeRequest).unwrap().event_type, "ping");
    });
}

/// @covers: EventSource::recv_next — closed bus returns Unavailable error
#[test]
fn test_event_receiver_recv_on_closed_bus_returns_unavailable_error() {
    use edge_application_event::InProcessEventBus;
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("rt");
    rt.block_on(async {
        let bus = InProcessEventBus::new(4);
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        drop(bus);
        let result = rx.recv_next(EventSourceRecvNextRequest).await;
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    });
}

/// @covers: EventSource::recv_next — noop bus receiver returns Unavailable immediately
#[test]
fn test_event_receiver_recv_noop_bus_returns_unavailable_edge() {
    use edge_application_event::NoopEventBus;
    let mut rx = NoopEventBus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
    let result = futures::executor::block_on(rx.recv_next(EventSourceRecvNextRequest));
    assert!(matches!(result, Err(EventError::Unavailable(_))));
}
