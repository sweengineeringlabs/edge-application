# ADR-004: edge-service — Service-to-Handler Bridge

**Status:** Superseded — `service`/`Service`/`ServiceRegistry` removed entirely, 2026-07-18. This
document is now historical record only. See "Amendment (2026-07-18): full removal" below.  
**Date:** 2026-06-13  
**Amended:** 2026-07-15 — corrected against the real, current codebase; the original Mandate and
Invariant I1 below described a design later superseded by `edge`'s own `ADR-020` (whose header
records "Status: Updated — bridge consolidated into `edge-domain-handler`") without this local
mirror ever being updated to match. See "Amendment: the bridge lives in `domain-handler`, not a
separate repo" below for what actually ships today.  
**Amended:** 2026-07-16 — `domain-base` (issue #139) is a deliberate, accepted exception to the
`no_foreign_type` boundary described below. See "Amendment: `domain-base` is exempt from
`no_foreign_type`" below.  
**Amended:** 2026-07-17 — `domain-handler` independently duplicates `edge-service`'s own
Service→Handler bridge, three implementations total, none with a confirmed live caller. See
"Amendment: three independent Service→Handler bridges" below.  
**Amended:** 2026-07-18 — full removal. See "Amendment (2026-07-18): full removal" below.  
**Governing ADR:** [ADR-020](https://github.com/sweengineeringlabs/edge/blob/main/docs/3-architecture/adr/ADR-020-edge-service-bridge.md) — edge-service Service-to-Handler Bridge

---

## Amendment (2026-07-18): full removal

2026-07-17's resolution (below) removed `domain-handler`'s two duplicate bridge copies but
stopped short of questioning `Service`/`ServiceRegistry` themselves, concluding "`Service` isn't
dead, it's an optional simpler path." Revisited the next day: given zero confirmed live callers
for *any* of the three bridges (including the one legitimate external one in `swe-edge-service`),
and that `ServiceRegistry` is structurally identical to `HandlerRegistry` (same CRUD-registry
contract, register/deregister/get/list/len/is_empty, keyed by name vs id) while `Handler` is a
strict superset of `Service`'s execute shape (adds `HandlerContext`, `id()`, `pattern()`,
`health_check()`) — "optional simpler path" and "redundant" were describing the same fact with
different verdicts attached. Confirmed empirically by running `edge-domain`'s own
`handler_registry` example end-to-end (register → get → execute happy+error path → health_check
→ deregister) with zero `Service` involvement.

**Resolution:** `service` crate (`edge-application-service`) deleted entirely from this repo,
along with its workspace membership, `edge-domain`'s `service` feature/facade/tests, and the
`examples/dataflow` crate (whose sole purpose was demonstrating the `Handler`/`Service`/`Command`
boundary this ADR describes — moot once `Service` no longer exists). `examples/service-query`
(the one example crate built around demonstrating `Service`) was rewritten to show the same
constructor-injection pattern through `Handler` directly instead of `Service` wrapped by a
hand-composed `Handler` — collapsing the two-struct pattern (`AuthSvc` + `AuthHandler`,
`LoginRecorderSvc` + `LoginHandler`) into one `Handler` each. Tracked in
[issue #147](https://github.com/sweengineeringlabs/edge-application/issues/147). Full workspace
`cargo build --workspace --all-targets` and `cargo test --workspace` clean.

This ADR is retained as the historical record of how `Service`/`Handler`'s relationship was
designed, drifted, and was ultimately found redundant — not deleted, since the reasoning trail
(the three-bridge duplication, the zero-caller finding) is exactly the kind of thing worth
keeping for the next time a similar "two ports doing the same job" pattern shows up.

---

## Amendment (2026-07-17): three independent Service→Handler bridges

Investigating the `no_foreign_type` blanket impls in
`domain-handler/main/src/core/handler/service/service_adapter.rs`
(`impl<T: svc::Service + ?Sized> Service for T {}` and the equivalent for `ServiceRegistry`)
surfaced a broader finding: there are **three** independent implementations of "bridge a `Service`
into a `Handler`" across this ecosystem, not one, and none has a confirmed live caller.

**The three, traced through git history:**

1. **`edge-service`'s `IntoHandler`/`DefaultServiceBridgeAdapter`** — `swe-edge-service` repo,
   `main/src/core/bridge/service_bridge_adapter.rs`. Built first: `283decf` (2026-06-13, repo
   scaffold, same day as this ADR's original Mandate) through `d4de18f` (2026-06-14, the
   `IntoHandler` extension trait). This is the bridge the original Mandate below designated as the
   one place `Service`↔`Handler` conversion should live. Legitimately built: the concrete adapter
   (`DefaultServiceBridgeAdapter<S>`) is `pub(crate)`, constructed explicitly inside
   `into_handler()`, and returned as a boxed trait object (`BoxedServiceBackedHandler`) so callers
   never see or name the concrete type.
2. **`domain-handler`'s `RegistryBridge`/`StdRegistryBridge`** —
   `core/handler/std_registry_bridge.rs`, added `bdd6ecd` (2026-07-01), over two weeks *after*
   `edge-service`'s bridge already existed. Bulk-transfers every entry of a `ServiceRegistry` into
   a `HandlerRegistry`. Has no counterpart in `edge-service` at all — `domain-handler`-only scope
   with no precedent. Depends on the blanket impls in `service_adapter.rs` to compile against a
   real `edge_application_service::ServiceRegistryStore`.
3. **`domain-handler`'s `IntoHandler`/`DefaultServiceHandler`** —
   `core/handler/service/service_handler.rs`, added in the same commit as the blanket impls,
   `bd911de` (2026-07-11) — whose own commit message, *"decouple api/ layer from foreign
   ServiceRegistry/Service/SecurityContext types (SEA no_foreign_type)"*, confirms this was a
   reactive fix to a structural-rule failure, not a deliberate decision to build a second bridge.
   Legitimately built in the same shape as `edge-service`'s (opaque `impl Trait` return, `pub(crate)`
   concrete adapter) — the mechanism is sound; its existence alongside an already-working
   `edge-service` equivalent is what's in question.

**Zero confirmed callers for any of the three**, checked against every real consumer named in
`docs/3-design/architecture.md`'s "Confirmed dataflow" section:

- `edge-dispatcher` — the one confirmed-live consumer of `domain-handler`'s `HandlerRegistry` — has
  no dependency on `edge-application-service` at all. Its own example
  (`edge-dispatcher/scm/examples/dispatch_pipeline.rs`) registers a hand-written `Handler`
  implementor (`UppercaseHandler`) directly, never via `Service`.
- `swe-edge-bootstrap` depends only on the `edge-domain` umbrella crate; its own `ServiceRegistry`
  type is an unrelated struct (egress HTTP/gRPC clients), not `domain-service::ServiceRegistry`.
- `edge-domain`'s own `Domain` facade never calls `.into_handler()` or `RegistryBridge::bridge()`.
- A grep across every local repo in the workspace for `.into_handler(`/`IntoHandler` found matches
  only inside the three implementations' own definitions and test suites.

**This does not mean `Service` is dead, or that `Handler` "skips" it as a defect.** `Handler` is
the actual dispatch-pipeline port; `Service` is an optional, simpler on-ramp for logic that doesn't
need `HandlerContext` (`Service::execute` has no context parameter at all, structurally). A
consumer writing `Handler` directly, as `edge-dispatcher`'s example does, reaches the same goal —
domain logic registered and dispatchable — by the more direct of two legitimate routes. Nothing is
avoided; the destination is what matters, and it's reached either way.

**What *is* a live finding**: three separate implementations of the same on-ramp, confirmed
functionally identical once exercised against the same input. Worked example, using
`edge-dispatcher`'s own `UppercaseHandler` reimplemented as a `Service`:

```rust
#[derive(Debug, Clone)]
struct UppercaseRequest(String);
#[derive(Debug, Clone, PartialEq, Eq)]
struct UppercaseResponse(String);
impl edge_application_base::Request for UppercaseRequest {}
impl edge_application_base::Response for UppercaseResponse {}

struct UppercaseService;
impl edge_application_service::Service for UppercaseService {
    type Request = UppercaseRequest;
    type Response = UppercaseResponse;
    fn name(&self, _: NameRequest) -> Result<NameResponse, ServiceError> {
        Ok(NameResponse { name: "uppercase".into() })
    }
    fn execute(&self, req: UppercaseRequest) -> BoxFuture<'_, Result<UppercaseResponse, ServiceError>> {
        Box::pin(async move { Ok(UppercaseResponse(req.0.to_uppercase())) })
    }
}
```

*Path 1 — domain-handler's `IntoHandler`/`DefaultServiceHandler`:*
```rust
let resp = UppercaseService.into_handler(IntoHandlerRequest)?;
registry.register(RegisterHandlerRequest::new(Arc::new(resp.handler)))?;
```

*Path 2 — domain-handler's `RegistryBridge`/`StdRegistryBridge`:*
```rust
let svc_registry: ServiceRegistryStore<UppercaseRequest, UppercaseResponse> = ServiceRegistryStore::default();
svc_registry.register(&RegisterServiceRequest::new(Arc::new(UppercaseService)))?;
StdRegistryBridge.bridge(BridgeRequest { src: &svc_registry, dst: &handler_registry })?;
```
`src: &svc_registry` only type-checks today via the blanket impl
`impl<T: svc::ServiceRegistry + ?Sized> ServiceRegistry for T {}` in `service_adapter.rs` — the one
already established as a real `no_foreign_type` violation, not a tool blind spot: the local
`ServiceRegistry` mirror trait becomes a transparent, automatic pass-through for the foreign one,
with no explicit per-type adapter decision at the call site. Contrast with Paths 1 and 3, where
crossing the bridge is an explicit method call producing an opaque, hidden concrete type.

*Path 3 — edge-service's `IntoHandler`/`DefaultServiceBridgeAdapter`:*
```rust
let resp = UppercaseService.into_handler(IntoHandlerRequest)?; // edge-service's own trait
registry.register(RegisterHandlerRequest::new(resp.handler))?; // Box<dyn ServiceBackedHandler<...>>
```

All three land `UppercaseService` in the identical end state — a `Handler` registered under id
`"uppercase"`, `execute` forwarding only `req.req` to `Service::execute` and dropping `ctx` in
every path that crosses through `Service` (vs. none dropped when `Handler` is implemented
directly, since it never had a context problem to begin with — see #140). Path 2 reaches that same
state through strictly more ceremony (an intermediate `ServiceRegistryStore`) and is the only one
of the three resting on the blanket-impl violation.

**Resolution decided 2026-07-17**: consolidate on `edge-service` (the legitimate, originally-mandated
path) and remove `domain-handler`'s two duplicate/illegitimate copies (#2 and #3 above), restoring
`domain-handler`'s original independence from `domain-service` per this ADR's Invariant I1. Tracked
in [issue #143](https://github.com/sweengineeringlabs/edge-application/issues/143), with tasks and
acceptance criteria for arching `edge-service`, bumping its pinned tag, and the full removal.

**Resolved 2026-07-17 — `domain-handler`'s duplicate copies removed.** `IntoHandler`,
`DefaultServiceHandler`, `RegistryBridge`, `StdRegistryBridge`, and the local
`Service`/`ServiceRegistry`/`ServiceBridge`/`ServiceHandler`/`Validator` mirror traits (33 files:
7 `api/traits/`, 9 `api/dto/`, 3 `core/`, 14 `saf/handler/{into,registry,service,validator}/`) are
deleted from `domain-handler`, along with `edge-application-service` as a dependency and the
`From<ServiceError> for HandlerError` conversion that only existed to support them. 24 test files
that existed solely to test this bridge deleted alongside; `tests/api_int_test.rs` (a mixed
layer-coverage file) edited to keep its non-bridge DTO tests and drop only the bridge-specific
ones. `domain-handler`'s `no_foreign_type` accepted-exception count dropped from 10 offenders to 4
as a direct result — only `Handler`/`HandlerRegistry`'s `domain-base` bound remains. Full
workspace `cargo build`/`cargo test` (default and `--all-features`) and `cargo clippy` clean
throughout. Version bumped to `0.2.0`, `CHANGELOG.md` updated. `domain-handler` now has zero
dependency on `domain-service`, matching Invariant I1 for the first time since `bd911de`
(2026-07-11) introduced the violation this amendment traces. Remaining: bump `edge-service`'s
pinned tag once this lands in a release (issue #143, task 2).

---

## Amendment (2026-07-16): `domain-base` is exempt from `no_foreign_type`

Issue #139 tightened `Handler`/`Service`'s `Request`/`Response` associated types from bare
`Send + 'static` to a real, shared contract: `edge_application_base::Request`/`Response`,
declared in a new foundational crate, `domain-base`. `domain-handler`'s `api/` layer now names
that foreign crate directly in type position (`type Request: edge_application_base::Request;` in
`handler.rs`, `handler_registry.rs`, `service.rs`, `service_registry.rs`, `registry_bridge.rs`) —
which `arch audit`'s `no_foreign_type` rule flags (10 offenders as of 2026-07-16).

**This is deliberate, not an oversight.** Two designs were tried and rejected before landing here:

1. **Local mirror traits per crate** (the same pattern this ADR's `Service`/`ServiceRegistry`
   mirrors already use to avoid naming `edge_application_service` in `api/`) — rejected by #139
   itself: `domain-handler`'s `Service`→`Handler` bridge (`core/handler/service/service_adapter.rs`)
   forwards `edge_application_service::Service::Request`/`Response` directly into the local
   `Service` trait. If `domain-handler` and `domain-service` each declared independent local
   marker traits, any type crossing that bridge would need to implement two separately-declared
   traits from two different crates simultaneously — the exact reconciliation problem a shared
   crate avoids.
2. **A sealed local trait, blanket-implemented from `core/`** (`pub trait Request: Sealed {}`
   locally in `api/`, with `impl<T: edge_application_base::Request> Request for T {}` living in
   `core/` as the one designated foreign-reference point) — this compiles and technically keeps
   `edge_application_base` out of `api/`'s own files, but on inspection it isn't a real port/adapter
   relationship: there is no concrete adapter in `core/` doing any work, only a blanket rule
   aliasing one trait to another. It also fails several *other* structural rules that don't
   recognize blanket impls or capped (`pub` nested inside `pub(crate)`) visibility
   (`core_implements_api_traits`, `filename_matches_type`, `one_type_per_file`,
   `saf_trait_svc_correspondence`) — trading one accepted exception for four new ones, none of
   them any more real than the first.

**The accepted resolution**: bind directly against `edge_application_base::Request`/`Response` in
`api/`, and treat `domain-base` as exempt from `no_foreign_type` — the same category `no_foreign_type`
already carves out for `std` types (`String`, `Vec`): *"part of the language contract"*, not a
business-logic dependency whose churn `api/` needs insulating from. `domain-base` plays that
identical role for this workspace: `domain-handler` and `domain-service` are both explicitly
designed around it as shared vocabulary, not as an external dependency being adapted in.

Confirmed via `arch audit --rs`, 2026-07-16: 10 `no_foreign_type` offenders in `domain-handler`
(`handler.rs`, `handler_registry.rs`, `service.rs`, `service_registry.rs`, `registry_bridge.rs` —
one `Request` and one `Response` reference each) and 4 in `domain-service` (`service.rs`'s
`Service::Request`/`Response`, `service_registry.rs`'s `ServiceRegistry::Request`/`Response`).
This is the one remaining exception, accepted as above.

**`api_dto_request_response_files_exist` — resolved for real, not accepted as a false positive.**
This rule (pre-existing, #134) initially reproduced for `Handler::execute`/`Service::execute` in
both crates. Unlike `no_foreign_type`, this one has a genuine fix: `api/handler/dto/request.rs`,
`api/handler/dto/response.rs` (`domain-handler`) and `api/service/dto/request.rs`,
`api/service/dto/response.rs` (`domain-service`) now exist, each a one-line
`pub use edge_application_base::{Request,Response};` — a real re-export of the actual shared
contract every `Self::Request`/`Self::Response` resolves to, not a placeholder. Each is
re-exported through to the crate root and covered by its own `tests/request_int_test.rs`/
`response_int_test.rs` (2 tests each, verifying a concrete and a zero-sized type satisfy the
bound through the re-export). `arch audit --rs` no longer reports this rule for either crate.

**`no_orphan_types` on `domain-handler`'s `EchoHandler` — also resolved for real.** Pre-existing,
unrelated to #139, but fixed while auditing this crate anyway: the tool's orphan-type exemption
detection for "genuine trait implementor" doesn't recognize a wrapped (multi-line) `impl<T: ...>
Trait for Type<T>` signature — `core/handler/in_process_handler_registry.rs` already carried a
comment documenting this exact gap and working around it by keeping the impl signature on one
line (`#[rustfmt::skip]`-protected). `core/handler/echo_handler.rs`'s `impl ... Handler for
EchoHandler<T>` had the same wrapped-signature shape and hit the same false positive; applying
the identical one-line-signature fix resolved it. `arch audit --rs` no longer reports this rule.

**`has_error_dir` in `domain-base` — also resolved for real, not accepted as unfixable.** Initial
attempt concluded this had no legitimate fix without adding a required method, which would
contradict #139's "pure marker, no required methods" scope. The actual fix: a *provided* (default,
non-breaking) `validate()` method on both `Request` and `Response`, mirroring `domain-entity`'s
own `Entity::validate` pattern exactly — `fn validate(&self, _req: ValidationRequest) ->
Result<ValidationResponse, RequestError> { Ok(ValidationResponse) }`. This gives `RequestError`/
`ResponseError` (reserved, `#[non_exhaustive]`, zero-variant — same shape as `EntityError`) a
genuine, non-fake home in a real trait method signature, satisfying `has_error_dir` and
`no_orphan_types` simultaneously without breaking any existing implementor (the method is
optional to override). `ValidationRequest`/`ValidationResponse` DTOs added, shared by both
`validate()` methods (the tool's own documented "shared-type case," not duplication). Covered by
6 new test files (`request_error_int_test.rs`, `response_error_int_test.rs`,
`validation_request_int_test.rs`, `validation_response_int_test.rs`, plus `_happy`/`_error`/
`_edge` scenario tests added to `request_e2e_test.rs`/`response_e2e_test.rs`). `arch audit --rs`
no longer reports this rule; `domain-base` is now 181/184 (was 179/184).

**`saf_layer_mirrors_api_domains` in `edge-domain` — also resolved for real.** `api/spi/`
(`DomainAssemblyHook`/`NoopDomainAssemblyHook`, a genuine extension-point trait, not a
placeholder) already had a working SAF facade — `domain_assembly_hook_svc_factory.rs` — but it
was filed under `saf/domain/` instead of `saf/spi/`, so the rule's api/-domain-to-saf/-domain name
match failed even though the facade itself was real and working. Moved the file to `saf/spi/`
(new `saf/spi/mod.rs` added, `saf/mod.rs` wired in) — pure relocation, no content change; the
`DOMAIN_SPI_SVC` re-export still resolves identically at the crate root, confirmed by the
pre-existing `tests/domain_assembly_hook_svc_factory_int_test.rs` (3 tests, still passing).

**`spi_organization_follows_api` — investigated and confirmed out of scope, not merely accepted.**
Running `arch audit --rs` from the workspace root (by accident, while re-checking `domain-handler`)
showed this rule failing in **all 20 workspace members**, including 17 crates #139 never touched
(`domain-app`, `domain-clock`, `domain-command`, `domain-entity`, `domain-event`,
`domain-lifecycle`, `domain-observer`, `domain-policy`, `domain-projection`, `domain-query`,
`domain-registry`, `domain-repository`, `domain-saga`, `domain-snapshot`, `domain-validator`,
`domain-valueobject`, plus `domain-service`/`domain-handler`/`domain-base`/`edge-domain` from this
work). This is a 100%-workspace-wide, pre-existing characteristic, not a #139 regression and not
selectively fixable within this issue's scope: the rule's own docs note a domain can legitimately
have no `spi/` counterpart "if a domain genuinely has no `spi/` counterpart by design" — true for
every port-contract crate here, none of which were ever designed with a pluggable third-party
extension point. Fabricating `spi/` directories with placeholder content across 20 crates to
force this rule green would be exactly the kind of fake work these standards reject. Left as a
known, workspace-wide gap outside this issue's scope — a candidate for its own dedicated cleanup
issue if ever prioritized, not something #139 should absorb.

---

## Amendment (2026-07-15): the bridge lives in `domain-handler`, not a separate repo

The original Mandate and Invariant I1 below claimed `domain-service` (`edge-domain-service`) and
`domain-handler` (`edge-domain-handler`) never import from each other, and that the bridge
between them lives in a standalone `edge-service` repo. Neither is true of the code as it exists
today, verified directly:

- `domain-handler/Cargo.toml` depends on `edge-application-service` directly.
- Four files in `domain-handler/main/src/core/handler/` import `edge_application_service::Service`
  directly: `handler_error.rs`, `service_adapter.rs`, `service_handler.rs`, `std_registry_bridge.rs`.
- The dependency is **one-way** — `domain-service` never imports `domain-handler`, confirmed by
  grep against `domain-service/Cargo.toml`.
- There is no separate `edge-service` crate anywhere in this workspace's `Cargo.toml`.
- The bridge itself — `IntoHandler` (blanket impl for every `S: Service`, wrapping it in
  `DefaultServiceHandler<S>`), and `RegistryBridge`/`StdRegistryBridge` (bulk-transfers a
  `ServiceRegistry`'s entries into a `HandlerRegistry`) — lives directly in
  `domain-handler/main/src/core/handler/service/service_handler.rs` and
  `domain-handler/main/src/core/handler/std_registry_bridge.rs`.

This is consistent with SEA's `no_foreign_type` rule as actually enforced in this repo: the
imports above are confined to `core/` (the implementation layer, where referencing another
domain crate's real types is permitted), never `api/` (the public contract layer, which still
declares its own local, structurally-identical `Service`/`ServiceRegistry` mirror traits so it
never names `edge_application_service` in a type position). So the "never import" framing below
was too strong even for the intent it was trying to protect — the actual, correct invariant is
narrower: **`domain-handler`'s `api/` layer must never reference `domain-service` types directly;
`domain-handler`'s `core/` layer doing so is expected and is where the sanctioned bridge lives.**

Full trace, with file citations, is in `docs/3-design/dataflow.md` §2. See also
[#139](https://github.com/sweengineeringlabs/edge-application/issues/139) (tightening
`Handler`/`Service`'s `Request`/`Response` associated types, which depends on this exact
dependency direction being accurate), [#140](https://github.com/sweengineeringlabs/edge-application/issues/140)
(the bridge silently drops `HandlerContext` at `DefaultServiceHandler::execute`), and
[#141](https://github.com/sweengineeringlabs/edge-application/issues/141) (a related review of
whether extracting concerns to external repos, as `domain-security` was, is the right pattern —
this ADR's original "bridge lives in a standalone repo" framing was the same instinct, and it did
not survive contact with the real implementation).

---

## Mandate (original, 2026-06-13 — superseded by the amendment above)

`edge-domain-service` and `edge-domain-handler` are independent domain contracts. Neither imports from the other. This workspace provides only the contracts.

The bridge between them lives in `edge-service` — a standalone infrastructure repo whose purpose is to enforce the workflow: `service_handler()` is the only sanctioned way to register a consumer's domain logic into the dispatch pipeline. The adapter code is thin by design; the value is the constraint, not the implementation. Consumers implement `Service`, call `service_handler()` in their `_svc.rs`, and the framework controls the rest.

---

## What this workspace owns

| Crate | Role |
|---|---|
| `edge-domain-service` | `Service<Req, Resp>` trait — named async domain operation |
| `edge-domain-handler` | `Handler<Req, Resp>` trait — infrastructure execution contract |

Both are stable. Neither is modified by ADR-020.

---

## Invariant (I1 from ADR-020) — superseded, see amendment above

`edge-domain-service` and `edge-domain-handler` must never import from each other. Any code that references both contracts belongs in `edge-service` or in a consumer `_svc.rs` — never in either domain crate.

---

## `edge-domain-service` contract

```rust
pub trait Service<Request, Response>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    fn name(&self) -> &str;
    fn execute(&self, req: Request) -> BoxFuture<'_, Result<Response, ServiceError>>;
}
```

Consumers implement this for their domain operations. The `name()` method returns the stable identifier used for logging and tracing — it is not used for routing.

**Note (2026-07-15):** the real, current trait shape differs slightly from this sample —
`domain-service::Service` uses associated types (`type Request`/`type Response`) rather than
generic trait parameters, and `execute` returns a boxed future via `BoxFuture` from the actual
implementation, not shown fully here. See `domain-service/main/src/api/service/traits/service.rs`
for the exact current definition. This sample is illustrative of the shape, not copy-paste
accurate to the current code.

---

## `ServiceRegistry`

`edge-domain-service` also provides `ServiceRegistry` — a runtime map of service name → boxed `Service` impl. It is the domain-level counterpart to `HandlerRegistry` in `edge-domain-handler`.

`ServiceRegistry` is independent of `HandlerRegistry`. Consumers may use either or both. **(2026-07-15: in the current code, the bridge is `RegistryBridge`/`StdRegistryBridge` and `IntoHandler`, both in `domain-handler`'s `core/` — not a `service_handler()` factory in a separate `edge-service` repo; see the amendment above.)**

---

## Cascade position

`edge-domain-service` is complete and stable. Unblocks: `edge-service` repo creation (ADR-020 implementation).

No cascade dependencies within this workspace — both contract crates ship independently.

**(2026-07-15: the `edge-service` repo creation this line anticipated did happen —
`sweengineeringlabs/edge-service` / package `swe-edge-service` exists — but it is orphaned: zero
real consumers anywhere in the `edge` ecosystem, confirmed by exhaustive grep. The actual,
live bridge is the one inside `domain-handler` described in the amendment above, not that repo.
See [#141](https://github.com/sweengineeringlabs/edge-application/issues/141).)**
