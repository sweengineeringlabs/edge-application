//! `ObserverContext` — composition handle bundling all observability primitives.
//!
//! Canonically defined in `edge-application-base`; re-exported here so
//! `edge_application_observer::ObserverContext` keeps resolving for existing consumers.
//! Use [`crate::StdObserveFactory::noop_observer_context`] in tests; wire an SDK-backed
//! implementation in production. See issue #145.

pub use edge_application_base::ObserverContext;
