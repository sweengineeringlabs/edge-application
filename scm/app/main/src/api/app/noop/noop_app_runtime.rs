/// Zero-config value that implements [`AppRuntime`](crate::api::AppRuntime).
///
/// Completes immediately with `Ok(())` without invoking the bootstrap,
/// intended for tests and structural wiring.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoopAppRuntime;
