//! Noop saga infrastructure — zero-sized markers for testing and bring-up.

mod noop_saga;
mod noop_saga_command;
mod noop_saga_event;

pub use noop_saga::NoopSaga;
pub use noop_saga_command::NoopSagaCommand;
pub use noop_saga_event::NoopSagaEvent;
