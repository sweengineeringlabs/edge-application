# edge-domain-base

The `Request`/`Response` marker-trait contract for `edge-domain`, shared by
`edge-domain-handler` and `edge-domain-service`.

`Send + 'static` alone is not a contract — any type satisfies it. `Request` and `Response`
give `Handler`/`Service` implementors a real, checkable bound instead of an unconstrained
associated type.

## Usage

```toml
[dependencies]
edge-application-base = { version = "0.1", path = "../domain-base" }
```

```rust
use edge_application_base::{Request, Response};

struct Greeting(String);
struct Farewell(String);

impl Request for Greeting {}
impl Response for Farewell {}
```

## License

MIT OR Apache-2.0
