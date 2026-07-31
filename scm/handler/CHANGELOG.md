# Changelog — edge-domain-handler

## [0.2.0] — 2026-07-17

### Removed
- **Breaking:** `IntoHandler`, `DefaultServiceHandler`, `RegistryBridge`, `StdRegistryBridge`,
  and the local `Service`/`ServiceRegistry`/`ServiceBridge`/`ServiceHandler`/`Validator` mirror
  traits (and their DTOs: `BridgeRequest`, `BridgeResponse`, `IntoHandlerRequest`,
  `IntoHandlerResponse`, `ServiceLookupRequest`, `ServiceLookupResponse`, `ListNamesRequest`,
  `ListNamesResponse`, `ValidatorRequest`) are removed from the public API. These implemented a
  `Service`→`Handler` bridge that duplicated a legitimate, already-existing bridge in the
  `swe-edge-service` repo — traced in `docs/adr/ADR-004-edge-service-bridge.md`'s 2026-07-17
  amendment to be a reactive duplicate (born from a `no_foreign_type` audit fix, not a deliberate
  design decision) with zero confirmed live callers anywhere in the workspace or its downstream
  consumers. Consumers wanting to bridge a `Service` into a `Handler` should depend on
  `swe-edge-service` directly.
- Removed the `edge-application-service` dependency entirely — `domain-handler` no longer
  references `domain-service` in any form, restoring the original independence invariant from
  ADR-004/upstream ADR-020.
- Removed the `SAF` identity constants that existed solely for this bridge:
  `INTO_HANDLER_SVC`, `INTO_HANDLER_SVC_FACTORY`, `REGISTRY_BRIDGE_SVC`,
  `REGISTRY_BRIDGE_SVC_FACTORY`, `SERVICE_SVC`, `SERVICE_SVC_FACTORY`, `SERVICE_BRIDGE_SVC`,
  `SERVICE_BRIDGE_SVC_FACTORY`, `SERVICE_HANDLER_SVC_FACTORY`, `SERVICE_REGISTRY_SVC`,
  `SERVICE_REGISTRY_SVC_FACTORY`, `BRIDGE_CONTEXT`, `MIN_SERVICE_NAME_LEN`,
  `VALIDATOR_SVC_FACTORY`.

### Fixed
- `no_foreign_type` accepted-exception count dropped from 10 offenders to 4 (only
  `Handler`/`HandlerRegistry`'s `Request`/`Response` bound against `domain-base` remains — the
  three files that made up the removed bridge no longer contribute to it).

## [0.1.2] — 2026-07-16

### Changed
- **Breaking:** `Handler`/`HandlerRegistry`'s `Request`/`Response` associated types, and the
  local `Service`/`ServiceRegistry` mirror traits' `Request`/`Response`, are now bound against
  the shared `edge_application_base::{Request, Response}` contract instead of bare
  `Send + 'static` (issue #139). Any type used as a payload for these traits must now
  `impl edge_application_base::Request` / `impl edge_application_base::Response` for itself —
  `Send + 'static` alone is no longer sufficient.
- Added `edge-application-base` as a new required dependency.

### Fixed
- Backfilled this changelog for the `0.1.1` version bump, which shipped without an entry.

## [0.1.0] — 2026-06-12

### Added
- Initial extraction from `edge-domain` monolith (issue #21)
- `Handler` trait: request/response execution units with registry and context
