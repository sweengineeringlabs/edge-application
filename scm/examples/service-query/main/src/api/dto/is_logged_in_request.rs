//! `IsLoggedInRequest` — `AuthHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct IsLoggedInRequest {
    pub session_token: String,
}
impl edge_application_base::Request for IsLoggedInRequest {}
