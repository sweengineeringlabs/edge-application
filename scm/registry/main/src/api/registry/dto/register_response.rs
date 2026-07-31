//! [`RegisterResponse`] — wrapper for successful registration.

/// Successful result of [`Registry::register`](crate::api::Registry::register).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RegisterResponse;
