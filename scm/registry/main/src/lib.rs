//! # edge-domain-registry
//!
//! The `Registry` port contract — an id-keyed resolution registry of shared
//! entries, generalizing the handler/service/task-controller family.
//!
//! Register a shared entry under a string id and resolve it later. The
//! reference implementation is the in-process [`MemoryRegistry`].

#![deny(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]

mod api;
mod core;
mod saf;

pub use saf::Registry;
pub use saf::REGISTRY_SVC;
pub use saf::REGISTRY_SVC_FACTORY;

// ── concrete reference types ──────────────────────────────────────────────────
pub use api::MemoryRegistry;

// ── request/response types ────────────────────────────────────────────────────
pub use api::DeregisterRequest;
pub use api::DeregisterResponse;
pub use api::EmptinessRequest;
pub use api::EmptinessResponse;
pub use api::LenRequest;
pub use api::LenResponse;
pub use api::ListIdsRequest;
pub use api::ListIdsResponse;
pub use api::RegisterRequest;
pub use api::RegisterResponse;
pub use api::RegistryLookupRequest;
pub use api::RegistryLookupResponse;
pub use api::TryRegisterRequest;
pub use api::TryRegisterResponse;

// ── error types ───────────────────────────────────────────────────────────────
pub use api::RegistryError;
