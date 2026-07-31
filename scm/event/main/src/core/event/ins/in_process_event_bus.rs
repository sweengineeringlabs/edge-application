//! [`EventBus`] impl for [`InProcessEventBus`] — tokio broadcast channel bus.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::{DomainEvent, EventBus, EventSource};
use crate::api::{
    EventBusPublishRequest, EventBusSubscribeRequest, EventBusSubscribeResponse,
    EventSourceRecvNextRequest, EventSourceRecvNextResponse, InProcessEventBus,
};

impl InProcessEventBus {
    /// Create a new bus with the given channel `capacity`.
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = tokio::sync::broadcast::channel(capacity);
        Self { sender }
    }
}

impl Default for InProcessEventBus {
    fn default() -> Self {
        Self::new(1024)
    }
}

/// Private receiver side of the broadcast channel.
struct InProcessEventBusSource {
    receiver: tokio::sync::broadcast::Receiver<Arc<dyn DomainEvent>>,
}

impl EventSource for InProcessEventBusSource {
    fn recv_next(
        &mut self,
        _req: EventSourceRecvNextRequest,
    ) -> BoxFuture<'_, Result<EventSourceRecvNextResponse, EventError>> {
        Box::pin(async {
            match self.receiver.recv().await {
                Ok(event) => Ok(EventSourceRecvNextResponse { event }),
                Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                    Err(EventError::Unavailable("channel closed".into()))
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                    Err(EventError::BroadcastLagged(n))
                }
            }
        })
    }
}

impl EventBus for InProcessEventBus {
    fn publish(&self, req: EventBusPublishRequest) -> BoxFuture<'_, Result<(), EventError>> {
        Box::pin(async move {
            // send() fails only when there are no receivers, which is not an error
            let _ = self.sender.send(req.event);
            Ok(())
        })
    }

    fn subscribe(
        &self,
        _req: EventBusSubscribeRequest,
    ) -> Result<EventBusSubscribeResponse, EventError> {
        Ok(EventBusSubscribeResponse {
            receiver: Box::new(InProcessEventBusSource {
                receiver: self.sender.subscribe(),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{EventTypeRequest, EventTypeResponse};

    struct InProcessEventBusTestEvt;
    impl DomainEvent for InProcessEventBusTestEvt {
        fn event_type(&self, _req: EventTypeRequest) -> Result<EventTypeResponse<'_>, EventError> {
            Ok(EventTypeResponse {
                event_type: "test.evt",
            })
        }
    }

    fn publish_req(event: Arc<dyn DomainEvent>) -> EventBusPublishRequest {
        EventBusPublishRequest { event }
    }

    /// @covers: publish
    #[test]
    fn test_publish_no_subscribers_returns_ok_happy() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let result = bus.publish(publish_req(Arc::new(InProcessEventBusTestEvt))).await;
            assert!(result.is_ok());
        });
    }

    /// @covers: publish
    #[test]
    fn test_publish_with_subscriber_delivers_event_happy() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let mut rx = bus
                .subscribe(EventBusSubscribeRequest)
                .expect("subscribe")
                .receiver;
            bus.publish(publish_req(Arc::new(InProcessEventBusTestEvt)))
                .await
                .expect("publish");
            let event = rx.recv_next(EventSourceRecvNextRequest).await.expect("recv").event;
            let event_type = event
                .event_type(EventTypeRequest)
                .expect("event_type")
                .event_type;
            assert_eq!(event_type, "test.evt");
        });
    }

    /// @covers: subscribe
    #[test]
    fn test_subscribe_two_receivers_independent_edge() {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio rt");
        rt.block_on(async {
            let bus = InProcessEventBus::new(16);
            let mut rx1 = bus
                .subscribe(EventBusSubscribeRequest)
                .expect("subscribe")
                .receiver;
            let mut rx2 = bus
                .subscribe(EventBusSubscribeRequest)
                .expect("subscribe")
                .receiver;
            bus.publish(publish_req(Arc::new(InProcessEventBusTestEvt)))
                .await
                .expect("publish");
            assert!(rx1.recv_next(EventSourceRecvNextRequest).await.is_ok());
            assert!(rx2.recv_next(EventSourceRecvNextRequest).await.is_ok());
        });
    }
}
