use crate::Interpreter;

mod standard;
pub use standard::*;

pub trait Library {
    fn register(&self, interpreter: &mut Interpreter);
}
