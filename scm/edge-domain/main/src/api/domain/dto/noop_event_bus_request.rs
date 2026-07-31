//! [`NoopEventBusRequest`] — request to construct a discarding [`EventBus`](crate::api::EventBus).

/// Request to construct an [`EventBus`](crate::api::EventBus) that silently discards all events.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NoopEventBusRequest;
