//! `GreeterResolutionRequest` — `GreeterResolutionHandler`'s `Self::Request`.

#[derive(Debug, Clone)]
pub struct GreeterResolutionRequest {
    pub id: String,
    pub name: String,
}
impl edge_application_base::Request for GreeterResolutionRequest {}
