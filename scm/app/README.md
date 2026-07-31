# edge-domain-app

Application boot contract for the edge domain.

## Traits

- **`Application`** — top-level lifecycle contract; implementors bring up all subsystems and hold the application open until shutdown.
- **`AppBootstrap`** — constructs an `Application` from a resolved service graph; the entry point for wiring all domain services.

## Usage

```rust
use edge_domain_app::{AppBootstrap, AppError, Application};
use futures::future::BoxFuture;

struct MyApp;

impl Application for MyApp {
    fn name(&self) -> &str { "my-app" }
    fn run(&self) -> BoxFuture<'_, Result<(), AppError>> {
        Box::pin(async {
            // start subsystems
            Ok(())
        })
    }
}

struct MyBootstrap;

impl AppBootstrap for MyBootstrap {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Ok(Box::new(MyApp))
    }
}
```
