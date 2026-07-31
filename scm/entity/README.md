# edge-domain-entity

The `Entity` port contract for `edge-domain`.

An entity has a stable identity that uniquely identifies it within its aggregate boundary.
Two entities are equal when their IDs are equal — not when all their fields match.

## Usage

```toml
[dependencies]
edge-domain-entity = { version = "0.1", git = "https://github.com/sweengineeringlabs/edge-domain" }
```

```rust
use edge_domain_entity::Entity;

struct OrderLine { id: u64, quantity: u32 }

impl Entity for OrderLine {
    type Id = u64;
    fn id(&self) -> &u64 { &self.id }
}
```

## License

MIT OR Apache-2.0
