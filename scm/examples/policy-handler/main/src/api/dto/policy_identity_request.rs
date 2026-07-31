//! `PolicyIdentityRequest` — `PolicyIdentityHandler`'s `Self::Request`.

#[derive(Debug, Clone, Copy, Default)]
pub struct PolicyIdentityRequest;
impl edge_application_base::Request for PolicyIdentityRequest {}
