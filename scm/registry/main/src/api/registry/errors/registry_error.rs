//! `RegistryError` — errors produced by a [`Registry`](crate::Registry).

use thiserror::Error;

/// Error produced when a registry operation cannot complete.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RegistryError {
    /// An entry is already registered under the given id, and a strict
    /// (`try_register`) registration was requested.
    #[error("an entry is already registered under id `{0}`")]
    DuplicateId(String),
}
