//! [`NoopCommand`] — a no-op [`Command`] structural placeholder.

/// A [`Command`] that does nothing and always returns `Ok(())`.
///
/// The implementation lives in `core::command::noop_command`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopCommand;
