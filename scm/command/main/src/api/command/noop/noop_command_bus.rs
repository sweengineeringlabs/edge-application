//! API-layer marker type for the noop command bus.

/// A [`CommandBus`](crate::CommandBus) that silently discards every dispatched command.
///
/// Use for testing and bring-up where a real bus is not needed.
/// The concrete `CommandBus` implementation lives in `core::command::noop_command_bus`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopCommandBus;
