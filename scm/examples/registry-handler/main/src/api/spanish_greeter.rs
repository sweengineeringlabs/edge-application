//! `SpanishGreeter` — a `Greeter` implementor registered under the `"es"` language code.

use crate::api::greeter::Greeter;

pub struct SpanishGreeter;
impl Greeter for SpanishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("¡Hola, {name}!")
    }
}
