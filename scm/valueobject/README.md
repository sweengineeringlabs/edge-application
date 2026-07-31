# edge-domain-valueobject

The `ValueObject` port contract and `NonEmptyString` reference implementation for `edge-domain`.

A value object has no identity — equality is determined by field values, not by object reference.

## Usage

```toml
[dependencies]
edge-domain-valueobject = { version = "0.1", git = "https://github.com/sweengineeringlabs/edge-domain" }
```

```rust
use edge_domain_valueobject::{NonEmptyString, ValueObject};

fn store<V: ValueObject>(v: V) { /* ... */ }

let name = NonEmptyString::new("Alice").unwrap();
store(name);
```

## License

MIT OR Apache-2.0
