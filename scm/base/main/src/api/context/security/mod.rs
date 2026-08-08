//! `Security` theme — the request principal, inherited directly from `edge-security-runtime`.
//!
//! `edge_security_runtime::SecurityContext` is re-exported as-is rather than mirrored behind a
//! local trait. `domain-base` treats `edge-security-runtime` as shared vocabulary this whole
//! workspace is built around -- the same `no_foreign_type` exemption category already granted to
//! `domain-base` itself for `domain-handler`/`domain-service` (see ADR-004's 2026-07-16
//! amendment) and carved out for `std` types (`String`, `Vec`). A local marker trait bought
//! decoupling from a type that was never going to be swapped, at the cost of every accessor
//! needing hand-written forwarding in a bridge (see git history, issue #152).

pub use edge_security_runtime::SecurityContext;
