//! SAF facade tests — `SecurityPrincipal` trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application_base::SecurityPrincipal;

struct AnonymousPrincipal;
impl SecurityPrincipal for AnonymousPrincipal {}

struct NamedPrincipal(#[allow(dead_code)] String);
impl SecurityPrincipal for NamedPrincipal {}

/// @covers: SecurityPrincipal — a local zero-field type can satisfy the marker trait
#[test]
fn test_security_principal_zero_field_type_satisfies_trait_happy() {
    let principal = AnonymousPrincipal;
    let _: &dyn SecurityPrincipal = &principal;
}

/// @covers: SecurityPrincipal — a type carrying data still satisfies the trait
#[test]
fn test_security_principal_data_carrying_type_satisfies_trait_error() {
    let principal = NamedPrincipal("svc-account".into());
    let _: &dyn SecurityPrincipal = &principal;
}

/// @covers: SecurityPrincipal — usable as a boxed trait object across a function boundary
#[test]
fn test_security_principal_boxed_trait_object_usable_edge() {
    fn accepts(_p: Box<dyn SecurityPrincipal>) {}
    accepts(Box::new(AnonymousPrincipal));
    accepts(Box::new(NamedPrincipal("x".into())));
}
