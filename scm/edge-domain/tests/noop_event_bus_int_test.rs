//! Coverage for api/event/types/noop/noop_event_bus.rs
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used, clippy::expect_used)]
use edge_application::DomainRuntime;
use edge_application::NoopEventBusRequest;
use edge_application::{
    Domain, EventBusPublishRequest, EventBusSubscribeRequest, EventSourceRecvNextRequest,
    NoopEventBus,
};
use futures::executor::block_on;
use std::sync::Arc;

#[test]
fn test_noop_event_bus_marker_type_is_constructible() {
    let marker = NoopEventBus;
    assert_eq!(std::mem::size_of_val(&marker), 0);
}

#[test]
fn test_noop_event_bus_publish_returns_ok() {
    block_on(async {
        use edge_application::DomainEvent;
        struct AnyEvent;
        impl DomainEvent for AnyEvent {}
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(())
        );
    });
}

#[test]
fn test_noop_event_bus_subscribe_returns_closed_receiver() {
    block_on(async {
        let bus = Domain.noop_event_bus(NoopEventBusRequest).unwrap().bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        // noop bus receiver immediately signals unavailable
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_err());
    });
}
