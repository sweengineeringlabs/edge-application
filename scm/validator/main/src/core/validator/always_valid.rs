//! [`Validator`] impl for [`AlwaysValid`].

use crate::api::AlwaysValid;
use crate::api::Validator;
use crate::api::ValidationRequest;
use crate::api::ValidationResponse;

impl Validator for AlwaysValid {
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, crate::api::ValidatorError> {
        Ok(ValidationResponse)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_always_returns_ok() {
        let result = AlwaysValid.validate(ValidationRequest);
        assert_eq!(result, Ok(ValidationResponse), "AlwaysValid should always succeed");
    }
}
