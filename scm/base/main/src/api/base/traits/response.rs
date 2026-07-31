//! `Response` — contract every `Handler`/`Service`-supplied response payload must satisfy.

use crate::api::base::dto::{ValidationRequest, ValidationResponse};
use crate::api::base::errors::ResponseError;

/// Marker bound for a `Handler::Response` or `Service::Response` associated type.
///
/// Implementors declare the concrete response payload they produce; this trait exists so the
/// contract has a real, checkable definition of "valid response" instead of an unconstrained
/// `Send + 'static` type parameter, which admits any type at all.
///
/// Shared by `domain-handler` and `domain-service` so a type crossing the `Service`→`Handler`
/// bridge only ever needs to satisfy one `Response` trait, not two independently-declared local
/// mirrors.
///
/// # Examples
///
/// ```rust
/// use edge_application_base::Response;
///
/// struct Farewell { name: String }
///
/// impl Response for Farewell {}
/// ```
pub trait Response: Send + 'static {
    /// Validate this response's invariants.
    ///
    /// Returns `Ok(ValidationResponse)` by default. Override to enforce domain rules.
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, ResponseError> {
        Ok(ValidationResponse)
    }
}
