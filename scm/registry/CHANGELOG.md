# Changelog — edge-domain-registry

## [0.1.0] — 2026-06-15

### Added
- `Registry` trait: id-keyed resolution registry of shared entries (ADR-029)
- `InMemoryRegistry`: in-process implementation; `try_register` rejects duplicate ids
- `RegistryFactory` / `StdRegistryFactory`: standard constructors
