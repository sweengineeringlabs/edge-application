//! [`OutboundRegisterRequest`] — request to register an outbound handle.

/// Request to register `handle` under `name`, replacing any existing entry.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct OutboundRegisterRequest<H> {
    /// The name to register the handle under.
    pub name: String,
    /// The handle to store.
    pub handle: H,
}
