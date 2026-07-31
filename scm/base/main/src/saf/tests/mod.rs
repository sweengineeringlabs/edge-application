//! Scaffold for external tests of `saf/` facade re-exports.
//!
//! `edge-application-base`'s `saf/` layer re-exports service identity constants only — it has
//! no standalone `pub fn` items to test here. Facade re-export coverage lives in the
//! crate-root `tests/` directory (e.g. `tests/request_svc_factory_int_test.rs`).
