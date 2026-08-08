//! `SecurityContext` — the authenticated request principal.
//!
//! Canonically re-exported (directly from `edge-security-runtime`, not mirrored behind a local
//! trait) by `edge-application-base`; re-exported here so `edge_application_handler::SecurityContext`
//! keeps resolving for existing consumers. See issue #145, #152.

pub use edge_application_base::SecurityContext;
