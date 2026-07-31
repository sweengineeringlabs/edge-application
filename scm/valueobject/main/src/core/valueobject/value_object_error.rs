//! `Display`/`Error` impls for [`ValueObjectError`].

use std::fmt;

use crate::api::ValueObjectError;

impl fmt::Display for ValueObjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "value must not be empty"),
            Self::Invalid(msg) => write!(f, "invalid value: {msg}"),
        }
    }
}

impl std::error::Error for ValueObjectError {}
