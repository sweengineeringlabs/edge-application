//! Core trait impls for [`NonEmptyString`].

use crate::api::NonEmptyString;
use crate::api::ValueObject;
use crate::api::ValueObjectError;

impl NonEmptyString {
    /// Construct a `NonEmptyString`.
    ///
    /// Returns [`ValueObjectError::Empty`] when `s` is an empty string.
    pub fn new(s: impl Into<String>) -> Result<Self, ValueObjectError> {
        let s = s.into();
        if s.is_empty() {
            Err(ValueObjectError::Empty)
        } else {
            Ok(Self(s))
        }
    }

    /// Return the contained string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl ValueObject for NonEmptyString {}
