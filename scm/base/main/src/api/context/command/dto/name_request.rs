//! [`NameRequest`] — input for [`Command::name`](super::super::traits::Command::name).

/// Request to resolve a [`Command`](super::super::traits::Command)'s stable name.
///
/// Carries no data today; it exists so `name` conforms to the uniform
/// `*Request` -> `Result<T, E>` port-contract shape, letting implementors
/// that resolve their name via a fallible lookup (e.g. a registry) fit the
/// same signature as every other dispatched operation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct NameRequest;
