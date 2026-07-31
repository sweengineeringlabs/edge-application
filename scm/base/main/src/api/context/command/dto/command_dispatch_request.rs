//! [`CommandDispatchRequest`] — input for [`CommandBus::dispatch`](super::super::traits::CommandBus::dispatch).
// @allow: dto_types_must_serialize — holds a live `Box<dyn Command>` to dispatch,
// not wire-format data; a trait object cannot derive Serialize/Deserialize.

use super::super::traits::Command;

/// Request to dispatch a [`Command`] through a [`CommandBus`](super::super::traits::CommandBus).
pub struct CommandDispatchRequest {
    /// The command to dispatch.
    pub command: Box<dyn Command>,
}
