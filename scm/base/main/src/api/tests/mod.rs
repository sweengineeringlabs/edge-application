//! Scaffold for external tests of standalone `pub fn` items declared directly in `api/`.
//!
//! `edge-application-base`'s `api/` layer exposes only trait contracts — it has no standalone
//! `pub fn` items to test here. Trait implementation coverage lives in the crate-root `tests/`
//! directory per [`trait_has_test_file`] and [`source_module_has_test_file`].
