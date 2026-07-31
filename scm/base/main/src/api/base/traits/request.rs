//! `Request` — contract every `Handler`/`Service`-supplied request payload must satisfy.

use crate::api::base::dto::{ValidationRequest, ValidationResponse};
use crate::api::base::errors::RequestError;

/// Marker bound for a `Handler::Request` or `Service::Request` associated type.
///
/// Implementors declare the concrete request payload they accept; this trait exists so the
/// contract has a real, checkable definition of "valid request" instead of an unconstrained
/// `Send + 'static` type parameter, which admits any type at all.
///
/// Shared by `domain-handler` and `domain-service` so a type crossing the `Service`→`Handler`
/// bridge only ever needs to satisfy one `Request` trait, not two independently-declared local
/// mirrors.
///
/// # Examples
///
/// ```rust
/// use edge_application_base::Request;
///
/// struct Greeting { name: String }
///
/// impl Request for Greeting {}
/// ```
pub trait Request: Send + 'static {
    /// Validate this request's invariants.
    ///
    /// Returns `Ok(ValidationResponse)` by default. Override to enforce domain rules.
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, RequestError> {
        Ok(ValidationResponse)
    }
}
