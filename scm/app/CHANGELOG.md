# Changelog — edge-domain-app

## [0.1.0] — 2026-06-25

### Added
- `Application` trait: top-level lifecycle boot contract for edge applications
- `AppBootstrap` trait: constructs an `Application` from a resolved service graph
- `AppError` enum: `BootFailed` and `CreationFailed` variants
- SAF identity constants: `APPLICATION_SVC`, `APPLICATION_SVC_FACTORY`, `APP_BOOTSTRAP_SVC`, `APP_BOOTSTRAP_SVC_FACTORY`
- Core `NoopApplication` and `NoopAppBootstrap` stub implementations
