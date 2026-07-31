//! [`OutboundDeregisterRequest`] — request to remove an outbound handle.

/// Request to remove the handle registered under `name`.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutboundDeregisterRequest {
    /// The name of the handle to remove.
    pub name: String,
}
