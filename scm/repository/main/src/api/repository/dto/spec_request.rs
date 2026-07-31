//! [`SpecRequest`] — request carrying a specification predicate.

use crate::api::repository::traits::Spec;

/// Request to filter entities of type `T` using `spec`.
pub struct SpecRequest<T> {
    /// The specification predicate to filter by.
    pub spec: Box<dyn Spec<Entity = T>>,
}
