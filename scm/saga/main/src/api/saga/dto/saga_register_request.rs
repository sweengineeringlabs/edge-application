//! [`SagaRegisterRequest`] — request to register a saga instance under an id.

/// Request to register `saga` under `id`, failing if `id` is already registered.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SagaRegisterRequest<Id, S> {
    /// The identifier to store the saga under.
    pub id: Id,
    /// The saga instance to register.
    pub saga: S,
}
