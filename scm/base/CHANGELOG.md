# Changelog — edge-domain-base

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
