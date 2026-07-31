//! [`SagaEventDescribeRequest`] — input for [`SagaEvent::describe`](crate::api::saga::traits::SagaEvent::describe).

/// Marker request; `describe` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct SagaEventDescribeRequest;
