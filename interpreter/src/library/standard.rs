use std::ops::Deref;

use crate::library::Library;
use crate::values::{Array, Value};
use crate::{Interpreter, NativeFunction, Object, ObjectConversion};

use std::str::FromStr;

pub struct StandardLibrary;

impl Library for StandardLibrary {
    fn register(&self, interpreter: &mut Interpreter) {
        let eval = NativeFunction::new(eval);
        interpreter
            .scope
            .push_var("eval", Value::NativeFunction(eval), false);

        interpreter.prototype_function("to_int", parse_string::<i64>);
        interpreter.prototype_function("to_float", parse_string::<f64>);
        interpreter.prototype_function("to_boolean", parse_string::<bool>);

        interpreter.prototype_function("length", array_length);
        interpreter.prototype_function("contains", array_contains);

        interpreter.prototype_function("length", string_length);
        interpreter.prototype_function("contains", string_contains);
    }
}

fn parse_string<O: FromStr + Object>(
    _: &mut Interpreter,
    this: String,
    _: Vec<Value>,
) -> Result<Value, String> {
    Ok(O::from_str(&this).map(O::into_value).unwrap_or(Value::Null))
}

fn eval(inter: &mut Interpreter, _: Value, mut args: Vec<Value>) -> Result<Value, String> {
    if args.len() != 1 {
        Err(format!("expected 1 argument, got {}", args.len()))
    } else {
        let code = std::mem::replace(&mut args[0], Value::Null);
        inter
            .eval(&String::try_get_as(code)?)
            .map_err(|err| format!("evaluation failed: {}", err.value))
    }
}

fn string_contains(
    _: &mut Interpreter,
    this: String,
    mut args: Vec<Value>,
) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("expected 1 argument, got {}", args.len()));
    }
    let arg = std::mem::replace(&mut args[0], Value::Null);
    let contains = this.contains(&String::try_get_as(arg)?);
    Ok(Value::Boolean(contains))
}

fn string_length(_: &mut Interpreter, this: String, _: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Integer(this.len() as i64))
}

fn array_length(_: &mut Interpreter, this: Array, _: Vec<Value>) -> Result<Value, String> {
    let len = this.0.deref().borrow().len();
    Ok(Value::Integer(len as i64))
}

fn array_contains(_: &mut Interpreter, this: Array, args: Vec<Value>) -> Result<Value, String> {
    let contains = match args.get(0) {
        Some(arg) => arg,
        None => return Ok(Value::Boolean(false)),
    };
    let result = this.0.deref().borrow().contains(contains);
    Ok(Value::Boolean(result))
}
