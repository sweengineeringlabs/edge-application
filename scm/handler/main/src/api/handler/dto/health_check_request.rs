//! [`HealthCheckRequest`] — request marker for [`Handler::health_check`](crate::api::handler::traits::Handler::health_check).

/// Request to check whether a handler is healthy and able to process requests.
pub struct HealthCheckRequest;
