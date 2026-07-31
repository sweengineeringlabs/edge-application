//! `ValueObjectError` — errors that value object construction can produce.

/// Errors returned when constructing a value object fails.
///
/// The canonical case is an invalid input (e.g., an empty string passed to
/// [`NonEmptyString::new`](crate::NonEmptyString::new)).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueObjectError {
    /// The supplied value was empty or blank.
    Empty,
    /// The supplied value failed a format constraint.
    Invalid(String),
}
