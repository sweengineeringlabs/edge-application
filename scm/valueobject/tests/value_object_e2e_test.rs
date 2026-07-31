//! SAF facade integration tests for `ValueObject` and `NonEmptyString`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::default_constructed_unit_structs
)]

use edge_application_valueobject::{NonEmptyString, ValidationRequest, ValueObject, ValueObjectError};

fn accepts_value_object<V: ValueObject>(_v: V) {}

#[derive(Clone, PartialEq, Eq, Hash)]
struct AlwaysValidVo;
impl ValueObject for AlwaysValidVo {}

#[derive(Clone, PartialEq, Eq, Hash)]
struct NeverValidVo;
impl ValueObject for NeverValidVo {
    fn validate(&self, _req: ValidationRequest) -> Result<(), ValueObjectError> {
        Err(ValueObjectError::Invalid("always invalid".to_string()))
    }
}

/// @covers: ValueObject::validate — default impl always succeeds
#[test]
fn test_validate_default_impl_returns_ok_happy() {
    AlwaysValidVo
        .validate(ValidationRequest)
        .expect("default validate impl should always succeed");
}

/// @covers: ValueObject::validate — overridden impl can fail
#[test]
fn test_validate_overridden_impl_returns_error_error() {
    assert!(NeverValidVo.validate(ValidationRequest).is_err());
}

/// @covers: ValueObject::validate — callable via the default ValidationRequest
#[test]
fn test_validate_default_request_is_zero_sized_edge() {
    assert_eq!(std::mem::size_of::<ValidationRequest>(), 0);
    assert!(AlwaysValidVo.validate(ValidationRequest::default()).is_ok());
}

/// @covers: ValueObject — NonEmptyString satisfies the ValueObject bound
#[test]
fn test_value_object_svc_non_empty_string_satisfies_value_object_bound_happy() {
    let v = NonEmptyString::new("test").unwrap();
    accepts_value_object(v);
}

/// @covers: ValueObject — NonEmptyString is accessible from crate root
#[test]
fn test_value_object_svc_non_empty_string_accessible_from_crate_root_happy() {
    let v = NonEmptyString::new("accessible").unwrap();
    assert_eq!(v.as_str(), "accessible");
}

/// @covers: ValueObjectError — error type is accessible from crate root
#[test]
fn test_value_object_svc_value_object_error_accessible_via_public_api_edge() {
    use edge_application_valueobject::ValueObjectError;
    let e = ValueObjectError::Empty;
    assert_eq!(e.to_string(), "value must not be empty");
}
