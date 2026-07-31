//! `EnglishGreeter` — a `Greeter` implementor registered under the `"en"` language code.

use crate::api::greeter::Greeter;

pub struct EnglishGreeter;
impl Greeter for EnglishGreeter {
    fn greet(&self, name: &str) -> String {
        format!("Hello, {name}!")
    }
}
