//! `SagaEvent` bridge — SEA Rule 121 api/core mirror.
//!
//! This path-level mirror lets the structural auditor match
//! `core/saga/saga_event.rs` (the blanket `DomainEvent -> SagaEvent` bridge)
//! to an api counterpart.

/// SEA Rule 121 marker — path co-location sentinel for the
/// `DomainEvent -> SagaEvent` blanket bridge in `core/saga/saga_event.rs`.
pub(crate) const _RULE_121: () = ();
