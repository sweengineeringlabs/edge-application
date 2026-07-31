//! # edge-domain-entity
//!
//! The `Entity` port contract — identity-bearing domain objects.
//!
//! An entity has a stable [`Id`](Entity::Id) that uniquely identifies it within
//! its aggregate boundary.  Equality is identity, not field equivalence.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Entity;
pub use saf::ENTITY_SVC;
pub use saf::ENTITY_SVC_FACTORY;

pub use api::EntityError;
pub use api::IdRequest;
pub use api::IdResponse;
pub use api::ValidationRequest;
pub use api::ValidationResponse;
