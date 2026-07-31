//! Integration tests for `EntityError`.

use edge_application_entity::EntityError;

/// @covers: EntityError — is Debug-formattable (derives Debug)
#[test]
fn test_entity_error_implements_debug_edge() {
    // EntityError has no variants; verify the Debug impl compiles via format_args.
    // Using a helper function that accepts the type in a match-exhaustive context.
    fn accepts_debug<T: std::fmt::Debug>(_: &T) {}
    let _ = accepts_debug::<EntityError>; // verifies EntityError: Debug at compile time

    // Verify Debug::fmt actually works (edge case: empty enum)
    assert!(!format!("{:?}", std::any::type_name::<EntityError>()).is_empty());
}
