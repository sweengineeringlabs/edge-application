# edge-application Domain Component Dataflow

The reference for how this workspace's domain crates actually connect to each other and to
downstream consumers, as traced and confirmed on 2026-07-15. Every claim below is backed by a
specific file read directly, not inferred from naming or documentation — see the citations
inline. Where something is real but *not yet* connected, that is stated explicitly rather than
implied.

**Amended 2026-07-18 (issue #145):** sections 3 and 4 described `handler`'s Observer/Command
bridges as local-mirror-trait + blanket-impl translations (`into_handler_error.rs`,
`ObserverContextAdapter`/`CommandBusAdapter`). That machinery was removed entirely — `handler`
now consumes `base`'s canonical `CommandBus`/`ObserverContext` traits directly, with zero bridge
code left. See the amendment notes at the top of sections 3 and 4 for the corrected picture.

**Amended 2026-07-18 (issue #147):** `service` (`Service`/`ServiceRegistry`) was removed from
this repo entirely, confirmed redundant with `handler` (`Handler`/`HandlerRegistry`) after an
audit found zero confirmed live callers for any Service→Handler bridge, in this repo or
externally. Sections 2 and 6, and the `examples/dataflow` crate they referenced (also removed),
are now historical record only — see the amendment notes at the top of each section.

---

## 1. The confirmed-live dispatch chain

```
Handler-implementing request
        │
        ▼
edge_dispatch::HandlerRegistryImpl        (edge-dispatcher, package alias `edge_dispatch`)
        │  — every method is a one-line forward, no logic of its own
        ▼
edge_application_handler::InProcessHandlerRegistry     (handler, THIS repo)
        │
        ▼
Handler::execute(ExecutionRequest { req, ctx })          (handler's own trait)
```

```mermaid
sequenceDiagram
    participant Server as AxumHttpServer / TonicGrpcServer
    participant Builder as RuntimeBuilder (swe-edge-bootstrap)
    participant Dispatcher as HttpHandlerRegistryDispatcher
    participant Impl as HandlerRegistryImpl (edge-dispatcher)
    participant Reg as InProcessHandlerRegistry (handler)
    participant Handler as Handler::execute (handler)

    Server->>Builder: incoming request
    Builder->>Dispatcher: dispatch(request)
    Dispatcher->>Impl: get(HandlerLookupRequest)
    Impl->>Reg: get(HandlerLookupRequest)
    Note right of Impl: pure forward — no logic of its own
    Reg-->>Impl: Arc<dyn Handler>
    Impl-->>Dispatcher: Arc<dyn Handler>
    Dispatcher->>Handler: execute(ExecutionRequest{req, ctx})
    Handler-->>Dispatcher: Result<Response, HandlerError>
    Dispatcher-->>Builder: response
    Builder-->>Server: response
```

**Proof:** `edge-dispatcher/scm/main/src/core/handler/handler_registry.rs` — `HandlerRegistryImpl<Request, Response>`
is `{ inner: InProcessHandlerRegistry<Request, Response> }`; `register`/`deregister`/`get`/`list_ids`/`len`
each forward directly to `self.inner`. This is the registry `swe-edge-bootstrap`'s
`RuntimeBuilder::http_route()`/`grpc_route()` actually constructs (`edge/scm/bootstrap/main/src/api/runtime/types/runtime_builder.rs`)
— the method wired to a live `AxumHttpServer`/gRPC server. So `handler` sits at the root
of the one dispatch path confirmed to run in production, regardless of any other abstraction
layer's documented-vs-real status (see `temp/edge-repo-dataflow-snapshot.md` for the separate
`edge-proxy` `Job`/`Router` question, which is outside this repo).

`Handler::execute`'s signature in this repo is:

```rust
async fn execute(&self, req: ExecutionRequest<'_, Self::Request>) -> Result<Self::Response, HandlerError>;

pub struct ExecutionRequest<'a, Req> {
    pub req: Req,
    pub ctx: &'a HandlerContext<'a>,
}

pub struct HandlerContext<'a> {
    pub security: &'a SecurityContext,
    pub commands: &'a dyn CommandBus,
    pub observer: &'a dyn ObserverContext,
}
```

This is the **split** `edge-domain-handler` lineage's shape (bundled `req`+`ctx`, three-field
context) — distinct from an older, undocumented-as-current "monolithic" lineage that still
exists in some consumer examples (`edge/scm/bootstrap/examples/hello_edge.rs`). See the
amendment added to `edge`'s ADR-024 (`edge/docs/3-architecture/adr/ADR-024-handler-execute-contract.md`,
2026-07-15) for the full account — that ADR documents the monolithic two-parameter shape only.

---

## 2. `Service` → `Handler`: real bridge, but no longer inside this repo

**Amended 2026-07-18 (issue #147): `service` removed entirely — this section is historical
record only.** `service`/`Service`/`ServiceRegistry` no longer exist anywhere in this repo,
including in `swe-edge-service` consumers this section describes. Removed as redundant with
`handler`/`Handler`/`HandlerRegistry`: an audit found zero confirmed live callers for any
`Service`→`Handler` bridge (internal or external), and `Handler` is a strict superset of
`Service`'s shape (adds `HandlerContext`, `id()`, `pattern()`, `health_check()`). See
`docs/adr/ADR-004-edge-service-bridge.md`'s final amendment for the full resolution.

**Amendment (2026-07-17, issue #143):** this section previously described `IntoHandler` and
`RegistryBridge`/`StdRegistryBridge` living in `handler` (`main/src/core/handler/std_registry_bridge.rs`).
That was a second, independently-built, duplicate of a bridge that already existed in the
`swe-edge-service` repo — three implementations of the same on-ramp in total, once `handler`'s
own two are counted. Both of `handler`'s copies were removed 2026-07-17, and
`handler` lost its `edge-application-service` dependency entirely (`handler/Cargo.toml`,
confirmed by grep — no `edge-application-service` entry remains). **As of this amendment,
`service::Service` and `handler::Handler` have zero bridge between them anywhere in
this repo.** See ADR-004's 2026-07-17 amendment for the full trace and worked example.

The sole surviving bridge is external, in the `swe-edge-service` repo (package `swe-edge-service`,
crate root `service/main/src/`), which depends on this repo's `handler`/`service` as
a downstream consumer — the same relationship `edge-dispatcher`/`swe-edge-bootstrap` have to
`handler` in section 1, not something this repo can see or verify from its own side (see
`architecture.md`'s scoping note). It is documented here only because the mechanism and its
consequences (next subsection) are directly relevant to understanding what a `Service` impl can
and cannot do once bridged — the same question this section always answered, just with the bridge
now correctly attributed to where it actually lives:

- **`IntoHandler`** (`swe-edge-service/service/main/src/core/bridge/service_handler.rs`) — a
  blanket impl for every `S: Service`, wrapping it in `DefaultServiceHandler<S>` whose
  `Handler::execute` delegates to the original `Service::execute`. Also now enforces
  `Validator::validate` at construction time (rejects a malformed `Service` before it is ever
  boxed into a `Handler`), rather than leaving validation as a separately-callable, easy-to-forget
  step — see that repo's `f3702f5` ("remove ServiceBridge/ServiceBackedHandler, enforce validation
  at construction").

```mermaid
sequenceDiagram
    participant App as Application startup (swe-edge-service consumer)
    participant Svc as S: Service (service contract)
    participant IH as IntoHandler (swe-edge-service, external to this repo)
    participant DSH as DefaultServiceHandler<S>

    rect rgb(235, 245, 255)
    Note over App,DSH: composition time — runs once, per Service instance
    App->>IH: svc.into_handler()
    IH->>DSH: validate() then box as Handler
    end

    participant Reg as HandlerRegistry (handler, THIS repo)
    participant Caller as Handler::execute caller (section 1's live chain)

    rect rgb(255, 240, 235)
    Note over DSH,Svc: per request — every call, via section 1's live chain
    Caller->>Reg: get(HandlerLookupRequest)
    Reg-->>Caller: Arc<dyn Handler> (the boxed DefaultServiceHandler<S>)
    Caller->>DSH: execute(ExecutionRequest{req, ctx})
    Note right of DSH: ctx (security/commands/observer)<br/>received here, but never forwarded below
    DSH->>Svc: execute(req.req)
    Svc-->>DSH: Result<Response, ServiceError>
    end
```

**What this means precisely:** a `Service` impl has no path to ever be reached by a live request
in *this* repo's own dispatch chain (section 1) unless some external consumer bridges it — that
bridging is no longer something `handler` offers internally. `service` and
`handler` are, within this repo, two fully independent port declarations with no
composition-time or per-request connection between them at all (see also section 6's related
`Command`↔`Service` finding, which follows from the same context-blind `Service::execute` shape
described next).

### `HandlerContext` does not survive the bridge

`Service::execute(&self, req: Self::Request) -> Result<Self::Response, ServiceError>` takes no
context parameter at all — `service`'s trait is context-blind by design, in this repo,
independent of who bridges it. Wherever the bridge lives, it cannot compensate for this.
`DefaultServiceHandler<S>::execute` — the `Handler` impl the bridge produces — receives the full
`ExecutionRequest<'_, S::Request> { req, ctx: &HandlerContext }` like any other handler, but only
forwards `req.req` to the wrapped service:

```rust
// swe-edge-service/service/main/src/core/bridge/service_handler.rs
async fn execute(&self, req: ExecutionRequest<'_, S::Request>) -> Result<S::Response, HandlerError> {
    self.inner.execute(req.req).await.map_err(Self::convert_error)   // req.ctx is never read
}
```

So `req.ctx` — `security`, `commands`, and `observer` alike — is silently dropped at this exact
line. A `Service` reached through this bridge cannot see the caller's `SecurityContext`, cannot
dispatch further commands through `ctx.commands`, and cannot emit traces/metrics through
`ctx.observer`, even though the `Handler` wrapping it was constructed with all three. This is not
a bug in the narrow sense — `Service`'s trait signature never promised context — but it is a real
constraint on what a bridged `Service` can do that is easy to miss from the `Handler` side, where
context is normally always available. Anything a `Service` impl needs from context has to be
supplied another way (constructor injection, a field on the concrete type), not through this
bridge. See issue #140.

`edge-llm`'s own ADR-085 (`edge-llm/docs/3-design/adr/ADR-085-memory-ports-service-dispatch-reachability.md`)
documents this exact mechanism independently, from the consumer side, and explicitly names an
earlier investigation in that repo that first concluded `service` was "an unused,
disconnected crate" before correcting itself against the real source — the same error this
document is written to avoid repeating.

---

## 3. `ObserverContext`: real bridge, not a stub

**Amended 2026-07-18 (issue #145): bridge removed, section below is historical record only.**
The seven blanket impls and `ObserverContextAdapter` described below no longer exist —
`handler/main/src/core/handler/observability/` (9 files) was deleted entirely, along with
`handler/main/src/api/handler/observer_context_adapter.rs`.
`HandlerContext.observer: &'a dyn edge_application_base::ObserverContext` now holds `base`'s
canonical trait directly (`handler/main/src/api/handler/traits/observer_context.rs` is a
one-line `pub use edge_application_base::ObserverContext;`) — any real `observer` implementor
(e.g. `StdObserveFactory`'s output) satisfies it with zero translation, since it's the same
trait, not a structurally-identical mirror.

`handler` depends directly on `observer` (`edge-application-observer` in
`handler/Cargo.toml`) and bridges it via seven blanket impls in
`handler/main/src/core/handler/observability/into_handler_error.rs`:

```rust
impl<T: obs::Counter + ?Sized> Counter for T { ... }
impl<T: obs::Gauge + ?Sized> Gauge for T { ... }
impl<T: obs::Histogram + ?Sized> Histogram for T { ... }
impl<T: obs::Span + ?Sized> Span for T { ... }
impl<T: obs::LogDrain + ?Sized> LogDrain for T { ... }
impl<T: obs::HandlerTracer + ?Sized> HandlerTracer for T { ... }
impl<T: obs::ObserverContext + ?Sized> ObserverContext for T { ... }
```

Any real `observer` implementor automatically satisfies handler's locally-declared
mirror traits (same `no_foreign_type` pattern as section 2). `ObserverContextAdapter<'a, T: ?Sized>(pub &'a T)`
exists only to wrap an already-erased `&dyn ForeignTrait` reference so it, too, can satisfy the
local trait via the same blanket impl — it is not itself an observer context, just a bridge
newtype. `HandlerContext.observer` is populated with this bridged value and is reachable on
*every* `Handler::execute` call (section 1) — but nothing makes using it mandatory; a handler
author must explicitly call `ctx.observer` to get tracing/logging/metrics. There is no
enforcement layer requiring it, unlike `ctx.commands` for writes (section 4).

---

## 4. `CommandBus`: the enforced write path

**Amended 2026-07-18 (issue #145): bridge removed, section below is historical record only; the
`examples/dataflow` reference below no longer exists (that crate was also removed, issue #147).**
`handler/main/src/api/handler/traits/command_bus.rs`, `.../traits/command.rs`, and
`.../dto/command_dispatch_request.rs` no longer declare local mirror types — each is now a
one-line `pub use edge_application_base::{CommandBus, Command, CommandDispatchRequest};`.
`handler/main/src/core/handler/command/` (the blanket-impl bridge, `into_handler_error.rs` +
`local_command_as_foreign.rs`) was deleted entirely. `ctx.commands.dispatch(...)` now dispatches
through the exact same `CommandDispatchRequest`/`Command` types `command`'s own
`DirectCommandBus` uses — there is no longer a "two distinct types with identical names" hazard
to warn about; `edge_application_command::Command` and `edge_application_handler::Command` are
the same trait.

`handler` also depends directly on `command`, bridged the same way section 3
bridges `observer`: `HandlerContext.commands: &'a dyn CommandBus` is `handler`'s own
locally-declared `CommandBus`/`Command`/`CommandDispatchRequest` mirror types
(`handler/main/src/api/handler/traits/command_bus.rs`, `.../traits/command.rs`,
`.../dto/command_dispatch_request.rs`), never `edge_application_command`'s directly (SEA
`no_foreign_type`). Any real `edge_application_command::Command`/`CommandBus`/`DirectCommandBus`
implementor satisfies the local mirror automatically via a blanket impl in
`handler/main/src/core/handler/command/into_handler_error.rs` — the same shape as section
3's `ObserverContext` bridge. Concretely: dispatching a command through `ctx.commands` from a
`Handler::execute` body means constructing `handler`'s own `CommandDispatchRequest { command:
Box<dyn edge_application_handler::Command> }`, not `edge_application_command`'s type of the same name — the two are
distinct, non-interchangeable types with identical names, easy to reach for the wrong one (see
`examples/dataflow/src/main.rs`, which hit exactly this mismatch as a compile error while being
built).

`HandlerContext.commands` is present on every `execute()` call. Per the (amended) ADR-024: a write
handler is expected to dispatch mutations through `ctx.commands.dispatch(...)` rather than
mutating a repository directly — this is a documented convention (`ADR-024` §"Write handlers —
enforced path"), not a type-level guarantee; nothing prevents a `Handler` impl from ignoring
`ctx.commands` entirely.

---

## 5. What is *not* connected — `registry::Registry<V>`

`registry` declares a generalized resolution-registry trait:

```rust
pub trait Registry<V: ?Sized + Send + Sync>: Send + Sync {
    fn register(&self, ...) -> Result<RegisterResponse, RegistryError>;
    fn get(&self, ...) -> Result<RegistryLookupResponse<Self::Value>, RegistryError>;
    // ... same shape as HandlerRegistry/ServiceRegistry
}
```

This crate exists specifically to generalize `HandlerRegistry` and `ServiceRegistry` — confirmed
by `edge`'s ADR-029 (`edge/docs/3-architecture/adr/ADR-029-registry-lifecycle-primitives.md`):
*"Generalize the resolution-registry family (Handler/Service/future-Task)."* But per that same
ADR: *"`HandlerRegistry`/`ServiceRegistry` are left unchanged in this ADR — refactoring them onto
`Registry` ripples through two published sub-crates and every consumer's pin graph, so it is a
later coordinated sweep, not blocking work."*

**Confirmed by exhaustive grep** (2026-07-15, across `registry`, `handler`,
`service`, `edge-domain`, and the entire `edge` monorepo): zero `impl From`/`Into`
conversions exist between `Registry<V>` and either `HandlerRegistry` or `ServiceRegistry`. The
only real consumer of `Registry<V>` today is the A2A plugin's task registry
(`edge/plugins/a2a`), unrelated to this repo's own `HandlerRegistry`/`ServiceRegistry`. Unlike
section 2's `Service`→`Handler` bridge, **no bridge exists for this unification** — that absence
is exactly why it remains disconnected. See the open tracking issue
[#139](https://github.com/sweengineeringlabs/edge-application/issues/139) for the related
(but distinct) `Request`/`Response` marker-trait tightening work, which touches the same two
crates (`handler`, `service`).

---

## 6. What is *not* connected — `Command`/`CommandBus` ↔ `Service`/`ServiceRegistry`

**Amended 2026-07-18 (issue #147): `service` removed entirely — this section is historical
record only.** The three-tier `Handler`/`Service`/`Command` context hierarchy below no longer
has a `Service` row; only `Handler` and `Command` remain. See section 2's amendment above.

No mechanism connects a dispatched `Command` to invoking a named `Service` from a
`ServiceRegistry`. This is not merely unwired — it is **structurally impossible** through the
existing `Service`→`Handler` bridge (section 2), because `Service::execute()`'s signature has no
context parameter to carry a `CommandBus` reference through in the first place.

**Confirmed by exhaustive grep** (2026-07-17, both directions, across `command`,
`service`, and `swe-edge-service`): zero references from `command`/`CommandBus` to
`Service`/`ServiceRegistry`, and zero references the other way. Neither crate's `Cargo.toml`
depends on the other (`command/Cargo.toml`, `service/Cargo.toml` — checked directly).

The reason isn't an oversight to fix — it falls out of a real, three-tier hierarchy of how much
execution context each of this repo's three request-handling ports actually receives:

| Port | `execute()` signature | Payload | Context available |
|---|---|---|---|
| `Handler` | `execute(&self, req: ExecutionRequest<'_, Req> { req, ctx: &HandlerContext })` | Yes | Yes — `security`, `commands: &dyn CommandBus`, `observer` (section 1) |
| `Service` | `execute(&self, req: Self::Request) -> Result<Self::Response, ServiceError>` | Yes | **None** — no context parameter exists at all (section 2) |
| `Command` | `execute(&self, _req: ExecutionRequest) -> Result<(), CommandError>` where `ExecutionRequest` is a zero-sized unit struct | **None** | **None** — carries neither payload nor context |

(`ExecutionRequest` here is `command`'s own type,
`command/main/src/api/command/dto/execution_request.rs` — `#[derive(...)] pub struct
ExecutionRequest;` — distinct from, and unrelated to, `handler`'s
`ExecutionRequest<'a, Req>` used in the table's `Handler` row.)

Tracing why a `Command` dispatched from inside a `Handler::execute` call can never reach a named
`Service`, even when that `Service` has separately been bridged into the very same
`HandlerRegistry` the `Handler` is running in:

1. `Handler::execute` receives `ctx.commands: &dyn CommandBus` and may call
   `ctx.commands.dispatch(CommandDispatchRequest { command })` (`command/main/src/api/command/traits/command_bus.rs`).
2. `CommandBus::dispatch` invokes `Command::execute(&self, _req: ExecutionRequest)` — but that
   `ExecutionRequest` is the zero-sized unit struct above, so the `Command` impl receives no
   payload and no context. It cannot look up or invoke a `Service` by name because it has nothing
   to look one up *with* — no `ServiceRegistry` reference, no `HandlerContext`, nothing.
3. Separately, even if a `Command` impl held a `ServiceRegistry` via constructor injection
   (the only way it could reach one at all, per step 2), and used it to fetch and bridge a
   `Service` into a `Handler` and call it — that inner `Service::execute(req)` call still has no
   context parameter (section 2's table row), so nothing round-trips back to the original
   `ctx.commands` the outer `Handler` started with. The two ports simply don't share a context
   shape that would let one meaningfully drive the other beyond one-way, hand-wired composition
   the type system doesn't help with.

So the gap is real at two independent levels: no existing code performs step 3's hypothetical
wiring (confirmed by the grep above), and even hand-written wiring following that path would still
lose context at the inner `Service::execute` boundary, for the same structural reason section 2
already documents for the `Handler`→`Service` direction. `Service::execute` lacking context isn't
a defect relative to some missing feature — it matches `service`'s own documented purpose
as pure application-layer logic, not infrastructure-aware execution; `Command` carrying neither
payload nor context is the most minimal of the three contracts by the same logic. The three rows
above are a coherent, intentional design tier, not three independent oversights that happen to
look similar.

---

## Summary table

| Connection | Status | Mechanism | Proof |
|---|---|---|---|
| `HandlerRegistryImpl` (edge-dispatcher) → `handler` | **Live, confirmed** | Direct struct wrapping, pure forwarding | `edge-dispatcher/.../handler_registry.rs` |
| `Service` (service) → `Handler` (handler) | **Removed — `service` crate deleted entirely (issue #147)** | N/A | `git log` — `service/` directory removed |
| `observer` → `handler` | **Real, per-request-reachable, not enforced — direct trait consumption, no bridge (issue #145)** | `HandlerContext.observer: &dyn edge_application_base::ObserverContext`, same trait `observer` implementors satisfy directly | `handler/main/src/api/handler/traits/observer_context.rs` |
| `command` → `handler` | **Real, per-request-reachable, convention-only — direct trait consumption, no bridge (issue #145)** | `HandlerContext.commands: &dyn edge_application_base::CommandBus`, same trait `command` implementors satisfy directly | `handler/main/src/api/handler/traits/command_bus.rs` |
| `registry::Registry<V>` → `HandlerRegistry`/`ServiceRegistry` | **Not connected; `ServiceRegistry` half moot — `service` removed (issue #147)** | None — deferred by ADR-029 | grep, exhaustive, zero matches |
| `Command`/`CommandBus` → `Service`/`ServiceRegistry` | **Removed — `service` crate deleted entirely (issue #147)** | N/A | `git log` — `service/` directory removed |

---

## See also

- `docs/3-design/temp/edge-repo-dataflow-snapshot.md` — the `edge` repo's own (partially stale)
  ingress/egress dataflow docs, temporarily mirrored here; delete once that repo's git conflicts
  are resolved and its docs are fixed in place.
- [Issue #139](https://github.com/sweengineeringlabs/edge-application/issues/139) — proposed
  `base` shared crate for `Request`/`Response` marker traits, touching the same
  `handler`/`service` boundary as section 2 above (historical — `service` since removed).
- [Issue #140](https://github.com/sweengineeringlabs/edge-application/issues/140) — `HandlerContext`
  dropped at the `Service`→`Handler` bridge; the same context-blind `Service::execute` shape was
  the root cause of section 6's `Command`↔`Service` finding (historical — `service` since removed).
- [Issue #143](https://github.com/sweengineeringlabs/edge-application/issues/143) — removal of
  `handler`'s duplicate `Service`→`Handler` bridge, resolved 2026-07-17; see ADR-004's
  amendment and section 2 above for the corrected picture (historical — `service` since removed
  entirely, issue #147).
- [Issue #145](https://github.com/sweengineeringlabs/edge-application/issues/145) — removed
  `handler`'s Command/Observer local-mirror-trait + blanket-impl bridge; `HandlerContext` now
  holds `base`'s canonical `CommandBus`/`ObserverContext` traits directly. Corrects sections 3
  and 4 above.
- [Issue #147](https://github.com/sweengineeringlabs/edge-application/issues/147) — full removal
  of `service`/`Service`/`ServiceRegistry` as redundant with `handler`/`Handler`/`HandlerRegistry`.
  Corrects sections 2 and 6 above; `examples/dataflow` (which demonstrated section 2/6's findings)
  was removed alongside it, since its entire purpose no longer applies.
- `edge`'s ADR-024 (amended 2026-07-15), ADR-020, ADR-029 — the governing ADRs for sections 1, 2,
  and 5 respectively.
- `edge-llm`'s ADR-085 — independent confirmation of section 2 from the consumer side (historical
  — `service` since removed).
