//! SAF facade tests — `SagaCommand` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::future::Future;
use std::pin::Pin;

use edge_application_saga::{SagaCommand, SagaCommandDispatchRequest, SagaError};
use futures::executor::block_on;

struct OkCmd;
impl SagaCommand for OkCmd {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), SagaError>> + Send + '_>> {
        Box::pin(async { Ok(()) })
    }
}

struct FailingCmd;
impl SagaCommand for FailingCmd {
    fn dispatch(
        &self,
        _req: SagaCommandDispatchRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), SagaError>> + Send + '_>> {
        Box::pin(async { Err(SagaError::CommandDispatchFailed("denied".into())) })
    }
}

/// @covers: SagaCommand::dispatch — success
#[test]
fn test_dispatch_ok_command_returns_ok_happy() {
    assert_eq!(block_on(OkCmd.dispatch(SagaCommandDispatchRequest)), Ok(()));
}

/// @covers: SagaCommand::dispatch — failure propagates
#[test]
fn test_dispatch_failing_command_returns_err_error() {
    assert_eq!(
        block_on(FailingCmd.dispatch(SagaCommandDispatchRequest)),
        Err(SagaError::CommandDispatchFailed("denied".into()))
    );
}

/// @covers: SagaCommand::dispatch — repeated dispatch is independent
#[test]
fn test_dispatch_repeated_calls_are_independent_edge() {
    let c = OkCmd;
    assert_eq!(block_on(c.dispatch(SagaCommandDispatchRequest)), Ok(()));
    assert_eq!(block_on(c.dispatch(SagaCommandDispatchRequest)), Ok(()));
}
