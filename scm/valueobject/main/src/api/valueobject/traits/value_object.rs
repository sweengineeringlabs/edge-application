//! `ValueObject` marker trait.

use std::hash::Hash;

use crate::api::valueobject::dto::ValidationRequest;
use crate::api::valueobject::errors::ValueObjectError;

/// Marker trait for value objects.
///
/// A value object has no identity of its own — two instances are equal when
/// all their fields are equal.  Implementors must be equality-comparable,
/// hashable, cloneable, and safe to share across threads.
///
/// Plain-data value objects with no trait implementors belong in the
/// `api/<theme>/vo/` directory of the theme that owns them. Concrete
/// implementors of this trait (which must not live in `vo/`, since `vo/`
/// holds data only) belong flat under their theme directory instead.
/// `NonEmptyString` ships as a reference implementation in this crate only.
pub trait ValueObject: Eq + Hash + Clone + Send + Sync {
    /// Re-check this value object's invariants.
    ///
    /// The default implementation always succeeds — override it when a
    /// value object can become invalid through a construction path other
    /// than its normal constructor (e.g. deserialization).
    fn validate(&self, _req: ValidationRequest) -> Result<(), ValueObjectError> {
        Ok(())
    }
}
