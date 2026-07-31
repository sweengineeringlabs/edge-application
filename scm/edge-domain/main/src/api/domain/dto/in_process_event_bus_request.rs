//! [`InProcessEventBusRequest`] — request to construct a broadcast-backed [`EventBus`](crate::api::EventBus).

use crate::api::EventBusConfig;

/// Request to construct an in-process broadcast-backed [`EventBus`](crate::api::EventBus).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InProcessEventBusRequest {
    /// Configuration for the broadcast channel backing the bus.
    pub config: EventBusConfig,
}
