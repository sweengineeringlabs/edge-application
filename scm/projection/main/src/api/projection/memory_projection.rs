use std::marker::PhantomData;

/// Builds a read model `R` by folding each event through a reducer `F`.
///
/// The `PhantomData<fn(&E)>` marker binds the event type without requiring
/// `E: Send + Sync` on the struct itself — those bounds are demanded only where
/// the [`Projection`](crate::Projection) impl needs them.
pub struct MemoryProjection<E, R, F> {
    pub(crate) read_model: R,
    pub(crate) reducer: F,
    pub(crate) _event: PhantomData<fn(&E)>,
}
