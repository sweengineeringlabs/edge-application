//! Scaffold for external tests of `saf/` facade re-exports.
//!
//! `edge-domain-event`'s `saf/` layer re-exports trait contracts and service
//! identity constants only — it has no standalone `pub fn` items to test here.
//! Facade re-export coverage lives in the crate-root `tests/` directory (e.g.
//! `tests/aggregate_svc_factory_int_test.rs`).
