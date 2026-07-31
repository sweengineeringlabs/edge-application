//! `GreeterRegistrationResponse` — `GreeterRegistrationHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct GreeterRegistrationResponse {
    pub registered: bool,
}
impl edge_application_base::Response for GreeterRegistrationResponse {}
