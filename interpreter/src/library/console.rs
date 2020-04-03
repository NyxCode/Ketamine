use crate::library::Library;
use crate::values::Object;
use crate::values::{NativeFunction, Value};
use crate::Interpreter;
pub struct Console;

impl Library for Console {
    fn register(&self, interpreter: &mut Interpreter) {
        let print_line = Value::NativeFunction(NativeFunction::new(print_line));
        interpreter.scope.push_var("print", print_line, false);
        let read_line = Value::NativeFunction(NativeFunction::new(read_line));
        interpreter.scope.push_var("read_line", read_line, false);
    }
}

fn print_line(_: &mut Interpreter, _: Value, args: Vec<Value>) -> Result<Value, String> {
    let content = args
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{}", content);
    Ok(Value::Null)
}

fn read_line(_: &mut Interpreter, _: Value, _: Vec<Value>) -> Result<Value, String> {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .map_err(|err| err.to_string())?;
    Ok(Value::String(line))
}
