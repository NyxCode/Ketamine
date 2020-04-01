use crate::library::Library;
use crate::values::{Array, Value};
use crate::Interpreter;

use std::ops::Deref;

pub struct StandardLibrary;

impl Library for StandardLibrary {
    fn register(&self, interpreter: &mut Interpreter) {
        interpreter.prototype_function("length", array_length);
        interpreter.prototype_function("contains", array_contains);
    }
}

fn array_length(this: Array, _: Vec<Value>) -> Result<Value, String> {
    let len = this.0.deref().borrow().len();
    Ok(Value::Integer(len as i64))
}

fn array_contains(this: Array, args: Vec<Value>) -> Result<Value, String> {
    let contains = match args.get(0) {
        Some(arg) => arg,
        None => return Ok(Value::Boolean(false)),
    };
    let result = this.0.deref().borrow().contains(contains);
    Ok(Value::Boolean(result))
}
