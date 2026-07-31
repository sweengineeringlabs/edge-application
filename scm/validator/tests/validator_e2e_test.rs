//! SAF facade tests — `Validator` trait exported from the crate root.

use edge_application_validator::{ValidationRequest, ValidationResponse, Validator, ValidatorError};

struct NonEmpty(String);
impl Validator for NonEmpty {
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, ValidatorError> {
        if self.0.is_empty() {
            Err(ValidatorError::Invalid("must not be empty".into()))
        } else {
            Ok(ValidationResponse)
        }
    }
}

/// @covers: Validator::validate — valid input
#[test]
fn test_validate_valid_input_returns_ok_happy() {
    assert_eq!(
        NonEmpty("hello".into()).validate(ValidationRequest),
        Ok(ValidationResponse)
    );
}

/// @covers: Validator::validate — returns Err for invalid input
#[test]
fn test_validate_empty_input_returns_err_error() {
    let result = NonEmpty("".into()).validate(ValidationRequest);
    assert!(result.is_err());
}

/// @covers: Validator::validate — works via dyn dispatch
#[test]
fn test_validate_via_dyn_trait_object_returns_ok_edge() {
    let v: &dyn Validator = &NonEmpty("x".into());
    assert_eq!(v.validate(ValidationRequest), Ok(ValidationResponse));
}
