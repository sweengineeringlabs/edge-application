//! # edge-domain-valueobject
//!
//! The `ValueObject` port contract and `NonEmptyString` reference implementation.
//!
//! A value object has no identity — equality is by field value.

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::NonEmptyString;
pub use api::ValidationRequest;
pub use api::ValueObjectError;
pub use saf::ValueObject;
pub use saf::VALUE_OBJECT_SVC;
pub use saf::VALUE_OBJECT_SVC_FACTORY;
