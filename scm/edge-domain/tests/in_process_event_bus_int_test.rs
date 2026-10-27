//! Coverage for api/event/types/ins/in_process_event_bus.rs
#![cfg(feature = "event")]
#![allow(clippy::unwrap_used)]

use edge_application::DomainRuntime;
use edge_application::InProcessEventBusRequest;
use edge_application::{
    Domain, DomainEvent, EventAggregateIdRequest, EventAggregateIdResponse, EventBusConfig,
    EventBusPublishRequest, EventBusSubscribeRequest, EventError, EventOccurredAtRequest,
    EventOccurredAtResponse, EventSourceRecvNextRequest, EventTypeRequest, EventTypeResponse,
    InProcessEventBus,
};
use futures::executor::block_on;
use std::sync::Arc;

#[derive(Clone)]
struct AnyEvent;
impl DomainEvent for AnyEvent {
    fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
        Ok(EventTypeResponse {
            event_type: "test.any",
        })
    }
    fn aggregate_id(
        &self,
        _req: EventAggregateIdRequest,
    ) -> Result<EventAggregateIdResponse<'_>, EventError> {
        Ok(EventAggregateIdResponse {
            aggregate_id: "agg-1",
        })
    }
    fn occurred_at(
        &self,
        _req: EventOccurredAtRequest,
    ) -> Result<EventOccurredAtResponse, EventError> {
        Ok(EventOccurredAtResponse {
            occurred_at: std::time::SystemTime::now(),
        })
    }
}

/// @covers InProcessEventBus — happy path: constructible via default
#[test]
fn test_in_process_event_bus_is_constructible_happy() {
    let bus = InProcessEventBus::default();
    assert_ne!(
        std::mem::size_of_val(&bus),
        0,
        "InProcessEventBus should be heap-backed"
    );
}

/// @covers InProcessEventBus — happy path: factory publishes successfully
#[test]
fn test_in_process_event_bus_factory_publishes_successfully_happy() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        assert_eq!(
            bus.publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await,
            Ok(())
        );
    });
}

/// @covers InProcessEventBus — error: publish with no active subscribers returns Ok
#[test]
fn test_in_process_event_bus_publish_no_subscribers_returns_ok_error() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        // No subscriber — dropped immediately. Publish must not error.
        let result = bus
            .publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent),
            })
            .await;
        assert_eq!(
            result,
            Ok(()),
            "publish with no active subscribers must return Ok"
        );
    });
}

/// @covers InProcessEventBus — edge: subscriber receives published event
#[test]
fn test_in_process_event_bus_subscriber_receives_published_event_edge() {
    block_on(async {
        let bus = Domain
            .in_process_event_bus(InProcessEventBusRequest {
                config: EventBusConfig::default(),
            })
            .unwrap()
            .bus;
        let mut rx = bus.subscribe(EventBusSubscribeRequest).unwrap().receiver;
        assert!(bus
            .publish(EventBusPublishRequest {
                event: Arc::new(AnyEvent)
            })
            .await
            .is_ok());
        assert!(rx.recv_next(EventSourceRecvNextRequest).await.is_ok());
    });
}
