use std::hash::Hash;

use crate::api::saga::errors::SagaError;
use crate::api::saga::traits::{SagaCommand, SagaEvent};
use crate::api::saga::dto::{
    SagaHandleRequest, SagaHandleResponse, SagaIsCompleteRequest, SagaIsCompleteResponse,
};

/// Orchestrates a long-running business process.
///
/// A saga reacts to domain events, maintains its own durable state, and
/// emits commands to drive the process toward completion or to compensate
/// on failure.  Dispatching the staged commands is the caller's responsibility.
pub trait Saga: Send + Sync {
    /// Unique identifier for this saga instance.
    type SagaId: Eq + Hash + Clone + Send + Sync;

    /// The union of events this saga handles.
    type Event: SagaEvent;

    /// Commands this saga can emit.
    type Command: SagaCommand;

    /// Apply an event; return zero or more commands to dispatch.
    fn handle(
        &mut self,
        req: SagaHandleRequest<'_, Self::Event>,
    ) -> Result<SagaHandleResponse<Self::Command>, SagaError>;

    /// Whether this saga has reached a terminal state (completed or compensated).
    fn is_complete(&self, req: SagaIsCompleteRequest) -> Result<SagaIsCompleteResponse, SagaError>;
}
