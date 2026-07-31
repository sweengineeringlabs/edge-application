//! [`ExecutionRequest`] — input for [`Command::execute`](super::super::traits::Command::execute).
//!
//! Canonically defined in `edge-application-base` as `CommandExecutionRequest`;
//! re-exported under this crate's original name for existing consumers. See issue #145.

pub use edge_application_base::CommandExecutionRequest as ExecutionRequest;
