//! # edge-domain-projection
//!
//! The `Projection` port contract — CQRS read-model builder driven by domain events.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::MemoryProjection;
pub use api::ProjectionApplyRequest;
pub use api::ProjectionError;
pub use api::ProjectionEventDescribeRequest;
pub use api::ProjectionEventDescribeResponse;
pub use api::ProjectionReadModelRequest;
pub use api::ProjectionReadModelResponse;
pub use api::TryDrainRequest;
pub use api::TryDrainResponse;
pub use edge_application_event::DomainEvent;
pub use saf::{
    Projection, ProjectionEvent, PROJECTION_EVENT_SVC, PROJECTION_EVENT_SVC_FACTORY, PROJECTION_SVC,
    PROJECTION_SVC_FACTORY,
};
