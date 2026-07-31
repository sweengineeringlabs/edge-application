//! [`InProcessEventBusResponse`] — a constructed broadcast-backed [`EventBus`](crate::api::EventBus) handle.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn EventBus>` handle,
// not wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::EventBus;

/// The [`EventBus`] constructed by [`DomainRuntime::in_process_event_bus`](crate::api::DomainRuntime::in_process_event_bus).
pub struct InProcessEventBusResponse {
    /// The broadcast-backed event bus.
    pub bus: Arc<dyn EventBus>,
}
