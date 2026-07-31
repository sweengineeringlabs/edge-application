# Changelog — edge-domain-observer

## [0.1.0] — 2026-06-18

### Added
- Initial scaffold: `HandlerTracer`, `MetricRegistry`, `LogDrain` port contracts (ADR-006)
- `StdObserveFactory` backed by noop primitives for local dev and unit testing
- SAF factory functions: `noop_handler_tracer`, `noop_metric_registry`, `noop_log_drain`, `std_observe_factory`
