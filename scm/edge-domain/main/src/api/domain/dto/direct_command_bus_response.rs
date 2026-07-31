//! [`DirectCommandBusResponse`] — a constructed [`CommandBus`](crate::api::CommandBus) handle.
// @allow: dto_types_must_serialize — holds a live `Arc<dyn CommandBus>` handle,
// not wire-format data; a trait object cannot derive Serialize/Deserialize.

use std::sync::Arc;

use crate::api::CommandBus;

/// The [`CommandBus`] constructed by [`DomainRuntime::direct_command_bus`](crate::api::DomainRuntime::direct_command_bus).
pub struct DirectCommandBusResponse {
    /// The inline-dispatching command bus.
    pub bus: Arc<dyn CommandBus>,
}
