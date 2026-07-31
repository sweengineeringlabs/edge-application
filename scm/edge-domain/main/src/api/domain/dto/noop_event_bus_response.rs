//! [`NoopEventBusResponse`] — a constructed no-op [`EventBus`](crate::api::EventBus) handle.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn EventBus>` handle,
// not wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::EventBus;

/// The [`EventBus`] constructed by [`DomainRuntime::noop_event_bus`](crate::api::DomainRuntime::noop_event_bus).
pub struct NoopEventBusResponse {
    /// The discarding event bus.
    pub bus: Arc<dyn EventBus>,
}
