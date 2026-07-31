//! [`ProjectionReadModelResponse`] — wrapper for the current read model.
// @allow: dto_types_must_serialize — holds a borrowed `&'a R` reference to the
// read model, not owned wire-format data; a derived Deserialize cannot produce
// a borrowed reference with an unbounded lifetime.

/// Result of [`Projection::read_model`](crate::api::projection::traits::Projection::read_model).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionReadModelResponse<'a, R> {
    /// The current read model.
    pub read_model: &'a R,
}
