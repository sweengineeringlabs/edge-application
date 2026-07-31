//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-domain-registry`'s only standalone public functions in `core/` are
//! [`MemoryRegistry::new`](crate::api::MemoryRegistry) and its `Default` impl, which are
//! exercised throughout the crate-root `tests/` directory (e.g. `tests/memory_registry_int_test.rs`,
//! `tests/registry_svc_int_test.rs`).
