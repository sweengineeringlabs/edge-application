//! Integration tests for `NoopEventPublisher`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_event::{
    DomainEvent, EventPublisher, EventPublisherPublishRequest, NoopEventPublisher,
};

struct SomeEvt;
impl DomainEvent for SomeEvt {}

/// @covers: NoopEventPublisher::publish — always returns Ok
#[test]
fn test_noop_event_publisher_publish_returns_ok_happy() {
    let result = futures::executor::block_on(
        NoopEventPublisher.publish(EventPublisherPublishRequest { event: &SomeEvt }),
    );
    assert_eq!(result, Ok(()));
}

/// @covers: NoopEventPublisher::publish — repeated calls never error
#[test]
fn test_noop_event_publisher_publish_repeated_never_errors_error() {
    for _ in 0..5 {
        assert_eq!(
            futures::executor::block_on(
                NoopEventPublisher.publish(EventPublisherPublishRequest { event: &SomeEvt })
            ),
            Ok(())
        );
    }
}

/// @covers: NoopEventPublisher::publish — via factory, dyn dispatch works
#[test]
fn test_noop_event_publisher_dyn_dispatch_ok_edge() {
    let pub_ = NoopEventPublisher;
    let evt: &dyn DomainEvent = &SomeEvt;
    assert_eq!(
        futures::executor::block_on(pub_.publish(EventPublisherPublishRequest { event: evt })),
        Ok(())
    );
}
