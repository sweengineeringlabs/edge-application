/// A no-op [`Saga`](crate::api::saga::traits::Saga) that never handles events.
///
/// Useful as a placeholder or test double where a concrete `Saga` type is
/// required but no actual processing should occur.
#[derive(Default)]
pub struct NoopSaga {
    pub(crate) complete: bool,
}
