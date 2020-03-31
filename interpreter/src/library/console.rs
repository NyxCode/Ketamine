use crate::library::Library;
use crate::values::Object;
use crate::values::{NativeFunction, Value};
use crate::Interpreter;
pub struct Console;

impl Library for Console {
    fn register(&self, interpreter: &mut Interpreter) {
        let print = Value::NativeFunction(NativeFunction::new(print));
        interpreter.scope.push_var("print", print, false);
    }
}

fn print(_: Value, args: Vec<Value>) -> Result<Value, String> {
    let content = args
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", content);
    Ok(Value::Null)
}
