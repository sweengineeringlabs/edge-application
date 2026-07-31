//! Scaffold for external tests of `saf/` facade re-exports.
//!
//! `edge-domain-policy`'s `saf/` layer re-exports trait contracts and service
//! identity constants only — it has no standalone `pub fn` items to test here.
//! Facade re-export coverage lives in the crate-root `tests/` directory (e.g.
//! `tests/policy_svc_factory_int_test.rs`).
