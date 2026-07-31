//! Basic `Validator` usage example.

use edge_application_validator::{AlwaysValid, ValidationRequest, ValidationResponse, Validator, ValidatorError};

struct Config {
    value: u32,
}

impl Validator for Config {
    fn validate(&self, _req: ValidationRequest) -> Result<ValidationResponse, ValidatorError> {
        if self.value > 0 {
            Ok(ValidationResponse)
        } else {
            Err(ValidatorError::Invalid("value must be positive".into()))
        }
    }
}

fn main() {
    println!("{:?}", Config { value: 1 }.validate(ValidationRequest));
    println!("{:?}", Config { value: 0 }.validate(ValidationRequest));
    println!("{:?}", AlwaysValid.validate(ValidationRequest));
}
