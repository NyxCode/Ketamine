use crate::library::Library;
use crate::values::{Array, Dictionary, Value};
use crate::Interpreter;
use std::borrow::BorrowMut;
use std::ops::Deref;

pub struct StandardLibrary;

impl Library for StandardLibrary {
    fn register(&self, interpreter: &mut Interpreter) {
        interpreter.prototype_function("length", array_length);
        interpreter.prototype_function("keys", object_keys);
    }
}

fn array_length(this: Array, _: Vec<Value>) -> Result<Value, String> {
    let len = this.0.deref().borrow().len();
    Ok(Value::Integer(len as i64))
}

fn object_keys(this: Dictionary, _: Vec<Value>) -> Result<Value, String> {
    let keys = this
        .0
        .deref()
        .borrow()
        .keys()
        .map(|key| Value::String(key.clone()))
        .collect::<Vec<_>>();

    Ok(Value::Array(Array::new(keys)))
}
