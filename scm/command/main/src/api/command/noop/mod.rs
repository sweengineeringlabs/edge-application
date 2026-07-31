//! Zero-sized noop marker types for testing and bring-up.

pub mod noop_command;
pub mod noop_command_bus;

pub use noop_command::NoopCommand;
pub use noop_command_bus::NoopCommandBus;
