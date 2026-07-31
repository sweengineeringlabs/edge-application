//! `IsLoggedInResponse` — `AuthHandler`'s `Self::Response`.

#[derive(Debug, Clone, Copy)]
pub struct IsLoggedInResponse {
    pub logged_in: bool,
}
impl edge_application_base::Response for IsLoggedInResponse {}
