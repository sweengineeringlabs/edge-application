//! `From` conversions folding `edge-application-base`'s error types into [`HandlerError`],
//! so `Handler::execute` implementors can propagate `HandlerContext.commands`/`.observer`
//! failures with a bare `?`. See issue #145.

use edge_application_base::{CommandError, ObserveError};

use crate::api::HandlerError;

impl From<CommandError> for HandlerError {
    fn from(err: CommandError) -> Self {
        HandlerError::ExecutionFailed(err.to_string())
    }
}

impl From<ObserveError> for HandlerError {
    fn from(err: ObserveError) -> Self {
        HandlerError::ExecutionFailed(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: From<CommandError> for HandlerError
    #[test]
    fn test_from_command_error_wraps_message_happy() {
        let err = CommandError::Internal("boom".to_string());
        assert_eq!(
            HandlerError::from(err),
            HandlerError::ExecutionFailed("internal: boom".to_string())
        );
    }

    /// @covers: From<CommandError> for HandlerError
    #[test]
    fn test_from_command_error_via_question_mark_operator_error() {
        fn convert(err: CommandError) -> Result<(), HandlerError> {
            Err(err)?;
            Ok(())
        }
        assert!(convert(CommandError::NotFound("x".into())).is_err());
    }

    /// @covers: From<ObserveError> for HandlerError
    #[test]
    fn test_from_observe_error_wraps_message_happy() {
        let err = ObserveError::NotInitialised;
        assert_eq!(
            HandlerError::from(err),
            HandlerError::ExecutionFailed("observe backend not initialised".to_string())
        );
    }

    /// @covers: From<ObserveError> for HandlerError
    #[test]
    fn test_from_observe_error_via_question_mark_operator_edge() {
        fn convert(err: ObserveError) -> Result<(), HandlerError> {
            Err(err)?;
            Ok(())
        }
        assert!(convert(ObserveError::BackendUnavailable("x".into())).is_err());
    }
}
