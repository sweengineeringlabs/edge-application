//! `GreeterRegistrationRequest` — `GreeterRegistrationHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct GreeterRegistrationRequest {
    pub id: String,
    pub language: String,
}
impl edge_application_base::Request for GreeterRegistrationRequest {}
