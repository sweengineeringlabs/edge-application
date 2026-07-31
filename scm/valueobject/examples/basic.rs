//! Basic `ValueObject` and `NonEmptyString` usage example.

use edge_application_valueobject::{NonEmptyString, ValueObject};

fn store<V: ValueObject>(_v: V) {}

fn main() {
    let Ok(name) = NonEmptyString::new("Alice") else {
        eprintln!("unexpected empty name");
        return;
    };
    store(name.clone());
    println!("name: {}", name.as_str());
}
