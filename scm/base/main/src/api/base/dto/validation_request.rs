//! [`ValidationRequest`] — zero-sized marker for requesting `Request`/`Response` validation.

/// Request to validate a `Request`/`Response` payload's invariants.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidationRequest;
