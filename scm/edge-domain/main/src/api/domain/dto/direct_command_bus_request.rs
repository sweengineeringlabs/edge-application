//! [`DirectCommandBusRequest`] — request to construct a [`CommandBus`](crate::api::CommandBus).

/// Request to construct a [`CommandBus`](crate::api::CommandBus) that dispatches commands inline.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DirectCommandBusRequest;
