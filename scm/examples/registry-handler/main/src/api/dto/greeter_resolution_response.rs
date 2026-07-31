//! `GreeterResolutionResponse` — `GreeterResolutionHandler`'s `Self::Response`.

#[derive(Debug, Clone)]
pub struct GreeterResolutionResponse {
    pub greeting: Option<String>,
}
impl edge_application_base::Response for GreeterResolutionResponse {}
