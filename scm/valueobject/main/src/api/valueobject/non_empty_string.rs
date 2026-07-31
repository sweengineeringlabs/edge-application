//! `NonEmptyString` — reference value object implementation.

/// A non-empty `String` that satisfies the [`ValueObject`] contract.
///
/// Equality and hashing are by value — two instances with the same content
/// compare equal.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NonEmptyString(pub(crate) String);
