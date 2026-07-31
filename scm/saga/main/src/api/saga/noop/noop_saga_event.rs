//! [`NoopSagaEvent`] — a no-op domain event for use with [`NoopSaga`](crate::api::saga::noop::NoopSaga).

/// A no-op domain event that carries no state and uses all default trait methods.
///
/// Used as the `Event` associated type for [`NoopSaga`].
#[derive(Clone)]
pub struct NoopSagaEvent;
