//! # edge-domain-validator
//!
//! The `Validator` port contract — structural field/configuration validation.
//!
//! A validator checks structural correctness (non-empty fields, range bounds, etc.)
//! rather than business intent (use a policy for domain rules).

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use api::AlwaysValid;
pub use api::ValidationRequest;
pub use api::ValidationResponse;
pub use api::ValidatorError;
pub use saf::Validator;
pub use saf::VALIDATOR_SVC;
pub use saf::VALIDATOR_SVC_FACTORY;
