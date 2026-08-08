# Changelog — edge-domain-base

## [0.3.0] — 2026-08-08

### Removed
- **Breaking:** the local `SecurityPrincipal` marker trait and its `core/context/security_bridge.rs`
  bridge impl for `edge_security_runtime::SecurityContext` (issue #152). `SecurityContext` is now
  re-exported directly from `edge-security-runtime` — `api::context::security` treats
  `edge-security-runtime` as shared vocabulary this workspace is built around, the same
  `no_foreign_type` exemption category already granted to `domain-base` itself (ADR-004's
  2026-07-16 amendment). This removes the accessor-forwarding tax the trait/bridge pattern
  required for every field exposed (`is_authenticated`, `subject`, `tenant_id`, `claim`, etc.) —
  consumers now read `SecurityContext`'s real fields/methods directly.
- Renamed SAF identity constants `SECURITY_PRINCIPAL_SVC`/`SECURITY_PRINCIPAL_SVC_FACTORY` to
  `SECURITY_CONTEXT_SVC`/`SECURITY_CONTEXT_SVC_FACTORY`.

### Changed
- `api::context::SecurityPrincipal` / `edge_application_base::SecurityPrincipal` replaced by
  `api::context::SecurityContext` / `edge_application_base::SecurityContext`, a direct re-export
  of `edge_security_runtime::SecurityContext`.

## [0.1.0] — 2026-07-16

### Added
- New crate (issue #139): `Request`/`Response` marker traits shared by `domain-handler` and
  `domain-service`, replacing each crate's independently-declared `Send + 'static` bound
- `Request::validate`/`Response::validate` — provided (default, non-breaking) methods returning
  `Result<ValidationResponse, RequestError>`/`Result<ValidationResponse, ResponseError>`,
  mirroring `domain-entity`'s `Entity::validate` pattern. `RequestError`/`ResponseError` are
  reserved, `#[non_exhaustive]`, currently uninhabited error namespaces for future use.
- `EmptyRequest`/`EmptyResponse` — canonical zero-sized "no payload" types, each independently
  implementing `Request`/`Response`. Not required to be paired with each other — a `Handler`/
  `Service` may combine either with any real type on the other side. Saves downstream crates
  from declaring their own local "no payload" marker for the same concept.
