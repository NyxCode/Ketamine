pub use console::*;
pub use standard::*;

use crate::Interpreter;

mod console;
mod standard;

pub trait Library {
    fn register(&self, interpreter: &mut Interpreter);
}
