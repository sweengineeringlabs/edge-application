//! Scaffold for external tests of standalone `pub fn` items declared directly in `core/`.
//!
//! `edge-domain-handler`'s only standalone public function in `core/` is
//! [`RegisterHandlerRequest::new`](crate::api::RegisterHandlerRequest::new), which is
//! exercised throughout the crate-root `tests/` directory (e.g. `tests/api_int_test.rs`,
//! `tests/in_process_handler_registry_int_test.rs`).
