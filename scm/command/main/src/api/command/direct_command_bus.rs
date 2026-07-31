//! `DirectCommandBus` — SEA Rule 121 api/core mirror.

/// Marker type describing a `CommandBus` that dispatches commands inline,
/// calling `cmd.execute()` directly in the same task with no queuing.
///
/// The concrete implementation lives in `core::command::direct_command_bus`.
pub struct DirectCommandBus;
