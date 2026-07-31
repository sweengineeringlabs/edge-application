//! [`SagaHandleResponse`] — wrapper for the commands staged by handling an event.

/// Result of [`Saga::handle`](crate::api::saga::traits::Saga::handle).
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SagaHandleResponse<C> {
    /// Commands to dispatch as a result of handling the event.
    pub commands: Vec<C>,
}
