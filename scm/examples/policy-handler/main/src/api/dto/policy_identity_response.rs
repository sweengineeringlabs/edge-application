//! `PolicyIdentityResponse` — `PolicyIdentityHandler`'s `Self::Response`.

#[derive(Debug, Clone)]
pub struct PolicyIdentityResponse {
    pub name: String,
}
impl edge_application_base::Response for PolicyIdentityResponse {}
