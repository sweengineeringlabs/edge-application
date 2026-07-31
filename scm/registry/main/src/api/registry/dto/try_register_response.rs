//! [`TryRegisterResponse`] — wrapper for successful strict registration.

/// Successful result of [`Registry::try_register`](crate::api::Registry::try_register).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TryRegisterResponse;
