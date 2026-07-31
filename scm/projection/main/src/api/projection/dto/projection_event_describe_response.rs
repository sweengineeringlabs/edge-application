//! [`ProjectionEventDescribeResponse`] — locally-owned summary of an event's identity.

/// Stable identifying metadata for an event consumed by a
/// [`Projection`](crate::api::projection::traits::Projection).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectionEventDescribeResponse {
    /// Stable type name for the event, e.g. `"order.created"`.
    pub event_type: String,
    /// ID of the aggregate that produced the event.
    pub aggregate_id: String,
}
