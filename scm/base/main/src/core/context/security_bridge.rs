//! Bridge from `edge_security_runtime::SecurityContext` to [`SecurityPrincipal`].
//!
//! `SecurityPrincipal` is declared in `api/` (SEA `no_foreign_type` forbids naming
//! `edge_security_runtime::SecurityContext` there directly); the concrete bridge to the
//! real, external principal type lives here in `core/`, same pattern used throughout this
//! workspace for foreign-type decoupling.

use edge_security_runtime::SecurityContext;

use crate::api::SecurityPrincipal;

impl SecurityPrincipal for SecurityContext {}
