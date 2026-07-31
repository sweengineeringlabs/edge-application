//! [`NoopEventPublisherResponse`] — a constructed no-op [`EventPublisher`](crate::api::EventPublisher) handle.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn EventPublisher>` handle,
// not wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::EventPublisher;

/// The [`EventPublisher`] constructed by [`DomainRuntime::noop_event_publisher`](crate::api::DomainRuntime::noop_event_publisher).
pub struct NoopEventPublisherResponse {
    /// The discarding event publisher.
    pub publisher: Arc<dyn EventPublisher>,
}
