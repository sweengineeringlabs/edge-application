//! `AlwaysMatchSpec` — a null-object [`Spec`](crate::Spec) that matches every entity.

use std::marker::PhantomData;

/// Reference [`Spec`](crate::Spec) implementation that matches every entity of type `T`.
///
/// Use as a null-object default where a `Spec` is required but no filtering
/// is needed (e.g. an unfiltered `find_by` that still goes through the
/// specification-based query path).
///
/// Construct via [`AlwaysMatchSpec::new`](crate::AlwaysMatchSpec); the concrete
/// trait impl lives in `core::repository::always_match_spec`.
pub struct AlwaysMatchSpec<T> {
    pub(crate) entity: PhantomData<fn() -> T>,
}
