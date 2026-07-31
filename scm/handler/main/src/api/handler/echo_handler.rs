//! [`EchoHandler`] — a handler that reflects its request back as the response.

use std::marker::PhantomData;

/// A handler that returns its request unchanged as the response.
///
/// Useful as a test double or default no-op implementation.
/// Construction and the [`Handler`](crate::api::handler::traits::Handler) impl live in
/// `core::handler::echo_handler`.
pub struct EchoHandler<T> {
    /// Stable identifier for this handler.
    pub id: String,
    /// Route pattern this handler matches.
    pub pattern: String,
    #[doc(hidden)]
    pub _marker: PhantomData<fn() -> T>,
}
