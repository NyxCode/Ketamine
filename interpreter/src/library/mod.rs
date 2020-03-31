use crate::Interpreter;

mod console;
mod standard;
pub use console::*;
pub use standard::*;

pub trait Library {
    fn register(&self, interpreter: &mut Interpreter);
}
