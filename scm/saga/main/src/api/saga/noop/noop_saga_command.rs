//! [`NoopSagaCommand`] — a no-op command for use with [`NoopSaga`](crate::api::saga::noop::NoopSaga).

/// A no-op command that executes immediately with `Ok(())`.
///
/// Used as the `Command` associated type for [`NoopSaga`].
pub struct NoopSagaCommand;
