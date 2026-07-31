//! E2E tests for `NonEmptyString`.
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use edge_application_valueobject::NonEmptyString;

/// @covers: NonEmptyString::new — valid input returns Ok
#[test]
fn test_non_empty_string_new_with_valid_value_returns_ok_happy() {
    let result = NonEmptyString::new("hello");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_str(), "hello");
}

/// @covers: NonEmptyString::new — empty input returns Err
#[test]
fn test_non_empty_string_new_with_empty_string_returns_err_error() {
    let result = NonEmptyString::new("");
    assert!(result.is_err());
}

/// @covers: NonEmptyString — equality is by value
#[test]
fn test_non_empty_string_equal_instances_are_eq_happy() {
    let a = NonEmptyString::new("value").unwrap();
    let b = NonEmptyString::new("value").unwrap();
    assert_eq!(a, b);
}

/// @covers: NonEmptyString — different content compares not-equal
#[test]
fn test_non_empty_string_different_content_are_not_eq_error() {
    let a = NonEmptyString::new("alpha").unwrap();
    let b = NonEmptyString::new("beta").unwrap();
    assert_ne!(a, b);
}

/// @covers: NonEmptyString — implements Clone
#[test]
fn test_non_empty_string_clone_equals_original_happy() {
    let original = NonEmptyString::new("cloneable").unwrap();
    let cloned = original.clone();
    assert_eq!(original, cloned);
}

/// @covers: NonEmptyString — usable as HashMap key (implements Hash + Eq)
#[test]
fn test_non_empty_string_usable_as_hashmap_key_happy() {
    let key = NonEmptyString::new("k1").unwrap();
    let mut map: HashMap<NonEmptyString, u32> = HashMap::new();
    map.insert(key.clone(), 42);
    assert_eq!(map.get(&key), Some(&42));
}
