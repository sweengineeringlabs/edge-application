//! `Entity` — identity-bearing domain object.

use std::hash::Hash;

use crate::api::entities::errors::EntityError;
use crate::api::entities::dto::{IdRequest, IdResponse, ValidationRequest, ValidationResponse};

/// A domain object with stable identity.
///
/// An entity has an [`Id`](Entity::Id) that uniquely identifies it within its
/// aggregate boundary.  Two entities are equal when their IDs are equal —
/// not when all their fields match.
///
/// Entities live *inside* aggregates.  The aggregate root is the consistency
/// boundary; entities it contains are not roots themselves.
///
/// # Examples
///
/// ```rust
/// use edge_application_entity::{Entity, IdRequest};
///
/// struct LineItem { id: u64, quantity: u32 }
///
/// impl Entity for LineItem {
///     type Id = u64;
///     fn id(&self, _req: IdRequest) -> Result<edge_application_entity::IdResponse<'_, u64>, edge_application_entity::EntityError> {
///         Ok(edge_application_entity::IdResponse { id: &self.id })
///     }
/// }
/// ```
pub trait Entity: Send + Sync {
    /// The type that uniquely identifies this entity within its aggregate.
    type Id: Eq + Hash + Clone + Send + Sync;

    /// Return the entity's stable identifier.
    fn id(&self, req: IdRequest) -> Result<IdResponse<'_, Self::Id>, EntityError>;

    /// Validate this entity's invariants.
    ///
    /// Returns `Ok(ValidationResponse)` by default. Override to enforce domain rules.
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, EntityError> {
        Ok(ValidationResponse)
    }
}
