# Changelog — edge-domain-lifecycle

## [0.1.0] — 2026-06-15

### Added
- `Lifecycle` trait: generic state-machine abstraction with pluggable `TransitionPolicy` (ADR-029)
- `ManagedLifecycle`: in-process implementation backed by `parking_lot::RwLock`
- `PermissivePolicy`: built-in transition policy that permits every state change
- `LifecycleFactory` / `StdLifecycleFactory`: standard constructors (`managed`, `permissive`)
- `LifecycleError::InvalidTransition`: returned when the active policy rejects a transition
