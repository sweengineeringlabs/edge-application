//! `SagaEvent` — local decoupling boundary for `Saga::Event`.

use crate::api::saga::errors::SagaError;
use crate::api::saga::dto::{SagaEventDescribeRequest, SagaEventDescribeResponse};
use crate::api::saga::noop::NoopSagaEvent;

/// The minimal contract a [`Saga`](super::Saga)'s associated `Event` type must satisfy.
///
/// Declared locally so `api/` never references `edge_application_event::DomainEvent`
/// directly in a type position (SEA `no_foreign_type`). Any `DomainEvent`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait SagaEvent: Send + Sync {
    /// Summarise this event's stable identity.
    fn describe(&self, req: SagaEventDescribeRequest) -> Result<SagaEventDescribeResponse, SagaError>;

    /// Return the canonical no-op event for tests and default wiring.
    fn noop() -> NoopSagaEvent
    where
        Self: Sized,
    {
        NoopSagaEvent
    }
}
