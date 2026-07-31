//! [`EventBus`] impl for [`NoopEventBus`] — discards all events.

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::{EventBus, EventSource};
use crate::api::{
    EventBusPublishRequest, EventBusSubscribeRequest, EventBusSubscribeResponse,
    EventSourceRecvNextRequest, EventSourceRecvNextResponse, NoopEventBus,
};

/// Private source returned by [`NoopEventBus::subscribe`] — immediately closed.
struct NoopEventBusSource;

impl EventSource for NoopEventBusSource {
    fn recv_next(
        &mut self,
        _req: EventSourceRecvNextRequest,
    ) -> BoxFuture<'_, Result<EventSourceRecvNextResponse, EventError>> {
        Box::pin(async { Err(EventError::Unavailable("noop bus has no events".into())) })
    }
}

impl EventBus for NoopEventBus {
    fn publish(&self, _req: EventBusPublishRequest) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async { Ok(()) })
    }

    fn subscribe(
        &self,
        _req: EventBusSubscribeRequest,
    ) -> Result<EventBusSubscribeResponse, EventError> {
        Ok(EventBusSubscribeResponse {
            receiver: Box::new(NoopEventBusSource),
        })
    }
}
