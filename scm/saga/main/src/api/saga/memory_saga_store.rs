use std::collections::HashMap;

use crate::api::saga::traits::Saga;

/// Stores [`Saga`] instances in a `HashMap` keyed by `SagaId`.
///
/// Reference implementation for development and testing; state is lost when
/// the process stops.
pub struct MemorySagaStore<S: Saga> {
    pub(crate) sagas: HashMap<S::SagaId, S>,
}
