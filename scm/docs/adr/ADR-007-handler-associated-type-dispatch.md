# ADR-007: Handler's Associated-Type Dispatch — Not Tower's Generic-Parameter Dispatch

**Status:** Accepted
**Date:** 2026-07-18
**Governing ADR:** None — this is a locally originated design clarification, not a mirror of an
`edge` platform decision. Recorded here because the question ("is `Handler` just this codebase's
version of Tower?") is easy to ask again, and the answer isn't obvious from the trait definition
alone.
**Relates to:** [ADR-004](ADR-004-edge-service-bridge.md) — `Service`'s (now-removed) associated-type
shape mirrored `Handler`'s exactly; see `docs/3-design/dataflow.md`.

---

## Context

`Handler` (`handler/main/src/api/handler/traits/handler.rs`) is this codebase's core dispatch
contract: an async request/response execution unit, looked up by id in a `HandlerRegistry`, and
invoked with a request-scoped `HandlerContext`. Described informally, that shape — "a trait with an
`execute`-like method, a registry that looks implementors up by name, request-scoped context
threaded in" — sounds like it could be describing [Tower](https://github.com/tower-rs/tower)'s
`Service<Request>`, the de facto standard shape for this kind of thing in the Rust ecosystem. It
is not the same mechanism, and the difference is deliberate, not an oversight or a simpler
reimplementation of Tower.

This distinction matters concretely within this platform, not just abstractly: `edge-proxy` (an
external repo consuming `edge-application-handler`) defines its own `Job<Request, Response>`
trait, which **does** use Tower's generic-parameter shape:

```rust
// edge-proxy, main/src/api/job/traits/job.rs
pub trait Job<Request = String, Response = String>: Send + Sync
where
    Request: Send + 'static,
    Response: Send + 'static,
{
    async fn run(&self, req: ExecutionRequest<'_, Request>) -> Result<JobResponse<Response>, JobError>;
    ...
}
```

So both shapes are real and both are currently in use in this platform, at two different layers.
Without this ADR, the natural next question — "should these two match?" — has no recorded answer,
and someone could plausibly "fix" one to look like the other, believing the mismatch is
accidental.

---

## Decision

`Handler` fixes `Request`/`Response` as **associated types**, not generic trait parameters:

```rust
#[async_trait]
pub trait Handler: Send + Sync {
    type Request: edge_application_base::Request;
    type Response: edge_application_base::Response;

    fn id(&self, _req: IdRequest) -> Result<IdResponse, HandlerError> { ... }
    fn pattern(&self, _req: PatternRequest) -> Result<PatternResponse, HandlerError> { ... }

    async fn execute(
        &self,
        req: ExecutionRequest<'_, Self::Request>,
    ) -> Result<Self::Response, HandlerError>;

    async fn health_check(&self, _req: HealthCheckRequest) -> Result<HealthCheckResponse, HandlerError> { ... }
}
```

This is the same idiom as `Iterator::Item` or `Future::Output`: one implementor, one fixed
associated `Request`/`Response` pair, enforced at compile time. A single concrete type — e.g. an
`AuthHandler` or a `CreateOrderHandler` — **is** one domain operation; it cannot simultaneously
implement `Handler` for two different request/response pairs. `HandlerRegistry<Req, Resp>` mirrors
this: the registry itself is generic over one fixed `Request`/`Response` pair for its whole
lifetime (confirmed in both this crate and `edge-dispatcher`'s wrapper over
`InProcessHandlerRegistry`), not per-entry.

Tower's `Service<Request>` puts `Request` on the trait as a **generic parameter** instead, which
means one struct can implement `Service<ReqA>` and `Service<ReqB>` simultaneously, via separate
`impl` blocks. That flexibility is the entire point of Tower's design: a `Timeout<S>` or `Retry<S>`
middleware must wrap *any* inner service regardless of what request type it carries, and the same
`Timeout<S>` type needs to be reusable across arbitrarily many, unrelated request shapes without
being rewritten per shape.

## Rationale

The two mechanisms answer different questions:

| Question | Mechanism | Used by |
|---|---|---|
| "This type *is* one specific operation — what request/response pair does it fix?" | Associated type | `Handler` |
| "This type is generic middleware — what request shape is it *currently* wrapping?" | Generic trait parameter | Tower's `Service<Request>`, `edge-proxy`'s `Job<Request, Response>` |

`Handler`'s job is domain-operation dispatch: each implementor is a committed, named piece of
business logic, and the one-implementor-one-operation invariant is a feature, not a limitation —
it's what lets `HandlerRegistry::get(id)` return something whose `Request`/`Response` types are
already known and fixed at the call site, no downcasting or type erasure required beyond the
`Arc<dyn Handler<Request = X, Response = Y>>` boundary itself.

`edge-proxy`'s `Job` sits one layer up, closer to Tower's own problem: a `Job` implementor's
purpose is to *route* — classify input, look up one of potentially many registered `Handler`s, and
dispatch to whichever one matches. That's a compositional, transport-facing concern, structurally
nearer to what Tower's generic parameter exists to support, which is why `edge-proxy` reached for
the same shape independently. (In practice, `edge-proxy`'s own example (`examples/dispatch.rs`)
still implements `Job` for exactly one concrete `Request`/`Response` pair — the generic parameter
is about what the trait *permits*, not a requirement that every implementor exploit it.)

Neither choice is a "simpler" or "more flexible" version of the other in the abstract — Tower's
shape is not strictly more powerful, it is shaped for a different job. Applying it to `Handler`
would weaken the one-operation-per-type guarantee this codebase's domain dispatch relies on;
applying `Handler`'s shape to something like `Job` would prevent exactly the "wrap any inner
service" composability that layer needs.

## Consequences

- Do not change `Handler`/`HandlerRegistry` to take `Request`/`Response` as generic trait
  parameters in the name of matching Tower or `edge-proxy`'s `Job`. That would allow one `Handler`
  implementor to serve multiple unrelated operations, which is not a goal here — it would be a
  regression against the one-operation-per-type invariant this ADR documents as deliberate.
- Do not treat `edge-proxy`'s `Job<Request, Response>` as "inconsistent" with `Handler` or in need
  of reconciliation. It is a different trait, at a different layer, solving a different problem —
  routing/composition vs. committed operation dispatch — and its shape is the appropriate one for
  that problem.
- When designing a future dispatch-related trait in this ecosystem, ask first: does one implementor
  correspond to exactly one fixed operation (→ associated type, `Handler`'s shape), or must one
  struct compose generically across many different, a priori unknown request shapes (→ generic
  trait parameter, Tower's/`Job`'s shape)? That question, not "what does Tower do," is what should
  decide the mechanism.

## See also

- `handler/main/src/api/handler/traits/handler.rs` — the current, exact `Handler` definition
- `edge-dispatcher`'s `docs/3-design/architecture.md` (external repo) — confirms `edge-dispatcher`
  re-exports `Handler` unmodified and never introduces a generic-parameter variant of it
- `edge-proxy`'s `docs/3-design/architecture.md` (external repo) — documents `Job`'s
  generic-parameter shape and its own upstream dependency on `Handler`/`HandlerContext`
- [ADR-004](ADR-004-edge-service-bridge.md) — `Service` (now removed, see its 2026-07-18 amendment)
  used the identical associated-type shape as `Handler`, for the identical reason: one implementor,
  one operation
