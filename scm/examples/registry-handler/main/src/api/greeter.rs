//! `Greeter` — the trait registered in and resolved from the injected `Registry<dyn Greeter>`.

pub trait Greeter: Send + Sync {
    fn greet(&self, name: &str) -> String;
}
