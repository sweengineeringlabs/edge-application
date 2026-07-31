//! SAF facade tests — `EventPublisher` trait via `NoopEventPublisher`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventPublisher, EventPublisherPublishRequest, NoopEventPublisher,
};

struct Evt;
impl DomainEvent for Evt {}

/// @covers: NoopEventPublisher::publish — returns Ok for any event
#[test]
fn test_publish_noop_publisher_returns_ok_happy() {
    let result = futures::executor::block_on(
        NoopEventPublisher.publish(EventPublisherPublishRequest { event: &Evt }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventPublisher — constructed via EventFactory
#[test]
fn test_noop_publisher_via_factory_returns_ok_happy() {
    let pub_ = NoopEventPublisher;
    let result = futures::executor::block_on(
        pub_.publish(EventPublisherPublishRequest { event: &Evt }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventPublisher::publish — dyn dispatch works
#[test]
fn test_publish_dyn_dispatch_returns_ok_edge() {
    let pub_: &dyn EventPublisher = &NoopEventPublisher;
    let evt: &dyn DomainEvent = &Evt;
    assert_eq!(
        futures::executor::block_on(pub_.publish(EventPublisherPublishRequest { event: evt })),
        Ok(())
    );
}

/// @covers: NoopEventPublisher::publish — repeated calls never error
#[test]
fn test_publish_repeated_calls_never_error_error() {
    for _ in 0..5 {
        let result = futures::executor::block_on(
            NoopEventPublisher.publish(EventPublisherPublishRequest { event: &Evt }),
        );
        assert_eq!(result, Ok(()));
    }
}
