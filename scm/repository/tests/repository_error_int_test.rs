//! Integration tests for `RepositoryError` display formatting.

use edge_application_repository::RepositoryError;

/// @covers: RepositoryError::NotFound — Display includes the key
#[test]
fn test_not_found_display_includes_key_happy() {
    let e = RepositoryError::NotFound("order-1".into());
    assert!(e.to_string().contains("order-1"));
}

/// @covers: RepositoryError::Conflict — Display includes the reason
#[test]
fn test_conflict_display_includes_reason_error() {
    let e = RepositoryError::Conflict("duplicate id".into());
    assert!(e.to_string().contains("duplicate id"));
}

/// @covers: RepositoryError — all variants format distinctly
#[test]
fn test_variants_format_distinctly_edge() {
    let nf = RepositoryError::NotFound("x".into()).to_string();
    let cf = RepositoryError::Conflict("x".into()).to_string();
    let un = RepositoryError::Unavailable("x".into()).to_string();
    let int = RepositoryError::Internal("x".into()).to_string();
    // All four variants must produce distinct strings
    assert_ne!(nf, cf);
    assert_ne!(nf, un);
    assert_ne!(nf, int);
    assert_ne!(cf, un);
    assert_ne!(cf, int);
    assert_ne!(un, int);
}
