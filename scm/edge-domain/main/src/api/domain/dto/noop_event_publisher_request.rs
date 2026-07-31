//! [`NoopEventPublisherRequest`] — request to construct a discarding [`EventPublisher`](crate::api::EventPublisher).

/// Request to construct an [`EventPublisher`](crate::api::EventPublisher) that discards all events silently.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NoopEventPublisherRequest;
