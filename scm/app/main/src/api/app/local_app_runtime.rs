//! `LocalAppRuntime` — the production `AppRuntime` implementation.

/// Executes the canonical `Bootstrap::build` → `Application::run` boot sequence in-process.
pub struct LocalAppRuntime;
