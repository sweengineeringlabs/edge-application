//! Core layer — `Handler` implementations wired to a shared `Registry<dyn Greeter>`.

mod greeter_registration_handler;
mod greeter_resolution_handler;

pub use greeter_registration_handler::GreeterRegistrationHandler;
pub use greeter_resolution_handler::GreeterResolutionHandler;
