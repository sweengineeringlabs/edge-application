//! [`ProjectionEventDescribeRequest`] — input for [`ProjectionEvent::describe`](crate::api::projection::traits::ProjectionEvent::describe).

/// Marker request; `describe` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectionEventDescribeRequest;
