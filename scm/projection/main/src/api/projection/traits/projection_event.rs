//! `ProjectionEvent` — local decoupling boundary for `Projection::Event`.

use crate::api::projection::errors::ProjectionError;
use crate::api::projection::dto::{ProjectionEventDescribeRequest, ProjectionEventDescribeResponse};

/// The minimal contract a [`Projection`](super::Projection)'s associated
/// `Event` type must satisfy.
///
/// Declared locally so `api/` never references `edge_application_event::DomainEvent`
/// directly in a type position (SEA `no_foreign_type`). Any `DomainEvent`
/// implementor satisfies this automatically via the blanket impl in `core/`.
pub trait ProjectionEvent: Send + Sync {
    /// Summarise this event's stable identity.
    fn describe(
        &self,
        req: ProjectionEventDescribeRequest,
    ) -> Result<ProjectionEventDescribeResponse, ProjectionError>;
}
