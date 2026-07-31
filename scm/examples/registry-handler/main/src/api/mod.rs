//! API layer — DTOs for the two `Handler`s wired to a shared `Registry<dyn Greeter>`.

mod dto;
mod english_greeter;
mod greeter;
mod spanish_greeter;

pub use dto::{
    GreeterRegistrationRequest, GreeterRegistrationResponse, GreeterResolutionRequest,
    GreeterResolutionResponse,
};
pub use english_greeter::EnglishGreeter;
pub use greeter::Greeter;
pub use spanish_greeter::SpanishGreeter;
