//! `SecurityPrincipal` trait — the authenticated request principal.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_handler::SecurityPrincipal` keeps resolving for existing consumers.
//! `edge_security_runtime::SecurityContext` already satisfies it via base's own bridge
//! (`base/core/context/security_bridge.rs`). See issue #145.

pub use edge_application_base::SecurityPrincipal;
