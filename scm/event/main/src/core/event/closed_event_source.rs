//! [`EventSource`] impl for [`ClosedEventSource`] — immediately returns `Unavailable`.

use futures::future::BoxFuture;

use crate::api::EventError;
use crate::api::{EventSource, EventSourceRecvNextRequest, EventSourceRecvNextResponse};
use crate::api::ClosedEventSource;

impl EventSource for ClosedEventSource {
    fn recv_next(
        &mut self,
        _req: EventSourceRecvNextRequest,
    ) -> BoxFuture<'_, Result<EventSourceRecvNextResponse, EventError>> {
        Box::pin(async { Err(EventError::Unavailable("event bus closed".into())) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ClosedEventSourceFixture;

    /// @covers: recv_next
    #[test]
    fn test_recv_next_closed_source_returns_unavailable_happy() {
        let mut src = ClosedEventSource;
        let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
        assert!(matches!(result, Err(EventError::Unavailable(_))));
    }

    /// @covers: recv_next
    #[test]
    fn test_recv_next_unavailable_message_contains_closed_error() {
        let mut src = ClosedEventSource;
        let err = match futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest)) {
            Err(e) => e,
            Ok(_) => panic!("expected Err"),
        };
        let msg = err.to_string();
        assert!(msg.contains("closed"), "expected 'closed' in: {msg}");
    }

    /// @covers: recv_next
    #[test]
    fn test_recv_next_repeated_calls_all_unavailable_edge() {
        let _ = ClosedEventSourceFixture;
        let mut src = ClosedEventSource;
        for _ in 0..3 {
            let result = futures::executor::block_on(src.recv_next(EventSourceRecvNextRequest));
            assert!(matches!(result, Err(EventError::Unavailable(_))));
        }
    }
}
