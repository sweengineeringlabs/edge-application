//! DTOs for the two `Handler`s wired to a shared `Registry<dyn Greeter>`.

mod greeter_registration_request;
mod greeter_registration_response;
mod greeter_resolution_request;
mod greeter_resolution_response;

pub use greeter_registration_request::GreeterRegistrationRequest;
pub use greeter_registration_response::GreeterRegistrationResponse;
pub use greeter_resolution_request::GreeterResolutionRequest;
pub use greeter_resolution_response::GreeterResolutionResponse;
