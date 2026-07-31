//! [`ExecutionRequest`] — input for [`Command::execute`](super::super::traits::Command::execute).

/// Request to run a [`Command`](super::super::traits::Command)'s write operation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExecutionRequest;
