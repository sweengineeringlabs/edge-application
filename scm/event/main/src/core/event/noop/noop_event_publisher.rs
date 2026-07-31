//! [`EventPublisher`] impl for [`NoopEventPublisher`] — silently discards all events.

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::EventPublisher;
use crate::api::{EventPublisherPublishRequest, NoopEventPublisher};

impl EventPublisher for NoopEventPublisher {
    fn publish(&self, _req: EventPublisherPublishRequest<'_>) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::DomainEvent;

    struct NoopEventPublisherTestEvt;
    impl DomainEvent for NoopEventPublisherTestEvt {}

    fn req(event: &dyn DomainEvent) -> EventPublisherPublishRequest<'_> {
        EventPublisherPublishRequest { event }
    }

    /// @covers: publish
    #[test]
    fn test_publish_any_event_returns_ok_happy() {
        let result =
            futures::executor::block_on(NoopEventPublisher.publish(req(&NoopEventPublisherTestEvt)));
        assert_eq!(result, Ok(()));
    }

    /// @covers: publish
    #[test]
    fn test_publish_via_dyn_returns_ok_error() {
        let pub_: &dyn EventPublisher = &NoopEventPublisher;
        let evt: &dyn DomainEvent = &NoopEventPublisherTestEvt;
        let result = futures::executor::block_on(pub_.publish(req(evt)));
        assert_eq!(result, Ok(()));
    }

    /// @covers: publish
    #[test]
    fn test_publish_repeated_calls_all_ok_edge() {
        for _ in 0..5 {
            assert_eq!(
                futures::executor::block_on(NoopEventPublisher.publish(req(&NoopEventPublisherTestEvt))),
                Ok(())
            );
        }
    }
}
