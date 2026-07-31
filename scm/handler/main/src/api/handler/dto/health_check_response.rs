//! [`HealthCheckResponse`] — response for [`Handler::health_check`](crate::api::handler::traits::Handler::health_check).

/// Whether a handler is healthy and able to process requests.
pub struct HealthCheckResponse {
    /// `true` if the handler is healthy.
    pub healthy: bool,
}
