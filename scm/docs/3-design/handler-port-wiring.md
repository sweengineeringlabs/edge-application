# Handler-to-Domain-Port Wiring

**Audience:** anyone implementing a `Handler` that needs to reach a repository, event store,
saga, snapshot store, projection, policy, or registry — i.e. almost every real `Handler`.

This documents the pattern established and proven in
[issue #149](https://github.com/sweengineeringlabs/edge-application/issues/149): how a `Handler`
reaches a domain port whose type is generic-per-use (`Repository<Entity, Id>`, `EventStore<Event>`,
etc.), since `HandlerContext` structurally cannot hold one.

---

## The problem

`HandlerContext { security, commands, observer }` holds exactly three fields, and can only ever
hold collaborators with this shape: **one canonical trait, shared identically by every `Handler`
in the process.** `SecurityPrincipal`, `CommandBus`, `ObserverContext` all qualify — every
`Handler` needs exactly one of each, and the concrete instance behind the trait is the same for
all of them.

Most real domain ports don't have that shape. `Repository<Entity, Id>` is generic per entity
type — a `HandlerContext` field typed `Repository<Entity, Id>` would need `Entity`/`Id` fixed at
the `HandlerContext` struct definition itself, but different handlers need different entities. A
single shared field can't be "a repository," only "a repository of `Order`s" — and the next
`Handler` needs "a repository of `User`s." The same is true of `EventStore<Event>`,
`SnapshotStore<AggregateId, Snap>`, `Projection<Event, ReadModel>`, `Policy<Input>`, and
`Registry<Value>`.

## The pattern: two seams, not one

1. **Constructor injection** for the `Handler`'s own fixed collaborator. The concrete port
   instance (`Arc<dyn Repository<Order, OrderId>>`, or similar) is a struct field on the
   `Handler` implementor itself, set once at construction — exactly like any other hexagonal
   adapter dependency. Different `Handler`s hold different concrete instances; the type is fixed
   *per handler*, not *per context*.
2. **`Self::Request`** for per-call data — the entity id, the event payload, the value to
   evaluate. This was already true for every `Handler` before this issue; the finding is that it
   is *also* the seam for reaching the port itself when the port's own method needs a per-call
   argument (e.g. `RepositoryIdRequest { id }`).

This is not a new mechanism — it is `examples/service-query`'s `AuthHandler`/`LoginHandler`
pattern (constructor-injected `QueryBus`/`CommandBus`), applied to the seven ports that don't fit
`HandlerContext`'s non-generic shape.

## Proven for all seven applicable ports, plus the two already-proven bus ports

| Port | Method signature shape | Wiring | Real error path exercised | Worked example |
|---|---|---|---|---|
| `CommandBus` | async, `&self` | `Arc<dyn CommandBus>` | — | `examples/service-query` (`LoginHandler`) |
| `QueryBus` | async, `&self` | `Arc<dyn QueryBus<Result = R>>` | — | `examples/service-query` (`AuthHandler`) |
| `Repository<Entity, Id>` | async, `&self` | `Arc<dyn Repository<...>>` | — (memory-backed `save` can't fail) | `examples/repository-handler` |
| `EventStore<Event>` | async, `&self` | `Arc<dyn EventStore<...>>` | — (memory-backed `append` with `ExpectedVersion::Any` can't fail; `Conflict` path exists but unexercised) | `examples/event-store-handler` |
| `Saga` | sync, `handle` needs `&mut self` | `Arc<Mutex<S>>` — **not** `SagaStore`, see below | — | `examples/saga-handler` |
| `SnapshotStore<AggregateId, Snap>` | async, `&self` | `Arc<dyn SnapshotStore<...>>` | `SnapshotError::InvalidVersion` (version 0) | `examples/snapshot-handler` |
| `Projection<Event, ReadModel>` | sync, `apply`/`try_drain` need `&mut self` | `Arc<Mutex<P>>` | `ProjectionError::EmptyStream` (empty batch) | `examples/projection-handler` |
| `Policy<Input>` | sync, `&self` | `Arc<dyn Policy<...>>` | `PolicyError` (composite rule violation) | `examples/policy-handler` |
| `Registry<Value>` | sync, `&self` | `Arc<dyn Registry<...>>` | `RegistryError::DuplicateId` | `examples/registry-handler` |

**Rule of thumb for which wiring shape applies:** if every method your `Handler` needs to call
takes `&self`, `Arc<dyn Port<...>>` is enough — the concrete implementation (`MemoryRepository`,
`MemoryEventStore`, `MemorySnapshotStore`, `MemoryRegistry`) handles its own interior mutability
via an internal lock. If any method you need takes `&mut self` (`Saga::handle`,
`Projection::apply`/`try_drain`), you need `Arc<Mutex<T>>` around the concrete type, and call
`.lock()` before the mutating call.

## A genuine structural finding, not just a wiring recipe: `SagaStore` can't support this

`saga` is the one port where the natural-looking approach — hold `Arc<dyn SagaStore<...>>>`,
`get()` a saga, call `.handle()` on it — **does not compile**, and not because of a missing
lock. `SagaStore::get<'a>(&'a self, ...) -> Result<SagaGetResponse<'a, Self::SagaInstance>,
SagaError>` returns an **immutable** borrow of the stored saga. `Saga::handle(&mut self, ...)`
needs a **mutable** one. No amount of locking the *store* fixes this — the `get` method's own
return type is the blocker, confirmed against both the trait definition and
`MemorySagaStore`'s concrete impl (which has no interior mutability of its own to route around
this with).

So `examples/saga-handler` wires `Handler` directly to a single `Saga` instance behind
`Arc<Mutex<S>>`, bypassing `SagaStore` entirely for the mutation path — not a workaround chosen
for convenience, but the only path that compiles given the trait as currently shaped. If
`SagaStore` should support get-then-mutate in the future, that's a trait redesign (an `&mut`
variant of `get`, or a `with_mut`-style closure-based accessor), tracked as a candidate follow-up
issue rather than folded into #149's scope.

## See also

- [Issue #149](https://github.com/sweengineeringlabs/edge-application/issues/149) — the issue
  this pattern was proven under, with a progress comment per port citing verification details
  (build/test/clippy output, the specific error path exercised).
- [ADR-007](../adr/ADR-007-handler-associated-type-dispatch.md) — why `Handler` fixes
  `Request`/`Response` as associated types in the first place; this document assumes that shape.
- `examples/service-query` — the original precedent this pattern generalizes from.
