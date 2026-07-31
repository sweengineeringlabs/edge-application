//! Integration tests for the `Entity` trait.

use edge_application_entity::{Entity, EntityError, IdRequest, IdResponse, ValidationRequest, ValidationResponse};

struct OrderLine {
    id: u64,
    #[allow(dead_code)]
    quantity: u32,
}

impl Entity for OrderLine {
    type Id = u64;
    fn id(&self, _req: IdRequest) -> Result<IdResponse<'_, u64>, EntityError> {
        Ok(IdResponse { id: &self.id })
    }
}

/// @covers: Entity::id
#[test]
fn test_id_returns_assigned_id_happy() {
    let line = OrderLine {
        id: 42,
        quantity: 3,
    };
    assert_eq!(*line.id(IdRequest).unwrap().id, 42u64);
}

/// @covers: Entity::id — two distinct IDs are not equal
#[test]
fn test_id_two_instances_different_ids_are_not_equal_error() {
    let a = OrderLine { id: 1, quantity: 5 };
    let b = OrderLine { id: 2, quantity: 5 };
    assert_ne!(a.id(IdRequest).unwrap().id, b.id(IdRequest).unwrap().id);
}

/// @covers: Entity::validate — default implementation always passes
#[test]
fn test_validate_default_impl_returns_ok_happy() {
    let line = OrderLine { id: 1, quantity: 1 };
    assert_eq!(line.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: Entity::validate — EntityError is currently uninhabited; no error path exists
#[test]
fn test_validate_no_error_variant_exists_error() {
    // EntityError has zero variants — validate() can only ever return Ok(())
    let line = OrderLine { id: 0, quantity: 0 };
    assert_eq!(line.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: Entity::validate — default validate passes for any id value
#[test]
fn test_validate_with_max_u64_id_passes_default_impl_edge() {
    let line = OrderLine { id: u64::MAX, quantity: 0 };
    assert_eq!(line.validate(ValidationRequest), Ok(ValidationResponse));
}

/// @covers: Entity::id — Id type parameter is generic (works with String)
#[test]
fn test_id_with_string_id_type_returns_correct_value_edge() {
    struct Tag {
        id: String,
    }
    impl Entity for Tag {
        type Id = String;
        fn id(&self, _req: IdRequest) -> Result<IdResponse<'_, String>, EntityError> {
            Ok(IdResponse { id: &self.id })
        }
    }
    let tag = Tag {
        id: "label-a".to_string(),
    };
    assert_eq!(tag.id(IdRequest).unwrap().id, "label-a");
}
