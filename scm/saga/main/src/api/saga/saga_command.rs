//! `SagaCommand` bridge — SEA Rule 121 api/core mirror.
//!
//! This path-level mirror lets the structural auditor match
//! `core/saga/saga_command.rs` (the blanket `Command -> SagaCommand` bridge)
//! to an api counterpart.

/// SEA Rule 121 marker — path co-location sentinel for the
/// `Command -> SagaCommand` blanket bridge in `core/saga/saga_command.rs`.
pub(crate) const _RULE_121: () = ();
