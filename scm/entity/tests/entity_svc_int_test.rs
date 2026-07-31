//! Integration tests for the entity SAF facade.

use edge_application_entity::{Entity, EntityError, IdRequest, IdResponse};

struct Point {
    id: u32,
}

impl Entity for Point {
    type Id = u32;
    fn id(&self, _req: IdRequest) -> Result<IdResponse<'_, u32>, EntityError> {
        Ok(IdResponse { id: &self.id })
    }
}

/// @covers: Entity — SAF facade exports Entity
#[test]
fn test_entity_svc_entity_is_accessible_via_public_api_happy() {
    let p = Point { id: 7 };
    assert_eq!(*p.id(IdRequest).unwrap().id, 7u32);
}

/// @covers: EntityError — reserved error type is accessible via public API
#[test]
fn test_entity_svc_entity_error_is_accessible_via_public_api_edge() {
    // EntityError is #[non_exhaustive] empty enum — verify it is importable.
    // The match below is exhaustive (no variants) and never panics.
    fn check(_e: EntityError) {
        // #[non_exhaustive] empty enum — wildcard required from external crates; body is intentionally empty
        #[allow(clippy::match_single_binding)]
        match _e {
            _ => {}
        }
    }
    let _check = check; // bind to confirm function compiles

    // Verify the re-exported type resolves to the same type declared in the errors module.
    assert!(std::any::type_name::<EntityError>().ends_with("EntityError"));
}
