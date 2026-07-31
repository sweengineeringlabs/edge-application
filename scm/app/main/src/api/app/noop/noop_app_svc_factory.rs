/// Zero-config value that implements [`AppServiceProvider`](crate::api::AppServiceProvider).
///
/// Returns a [`NoopAppBootstrap`](crate::api::NoopAppBootstrap) without any service wiring,
/// intended for tests and structural scaffolding.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoopAppSvcFactory;
