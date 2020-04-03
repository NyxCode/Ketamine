use crate::values::{Object, Value};
use crate::{Evaluate, Interpreter};

use parser::Pos;
use std::cell::RefCell;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Function {
    pub function: Rc<parser::ast::Function>,
}

impl Object for Function {
    fn type_name(&self) -> &'static str {
        "function"
    }

    fn into_value(self) -> Value {
        Value::Function(self)
    }

    fn to_string(&self) -> String {
        let args = self
            .function
            .params
            .iter()
            .map(|arg| arg.value.0.as_str())
            .collect::<Vec<&str>>()
            .join(", ");
        format!("function({}) {{..}}", args)
    }

    fn equal(&self, other: &Value) -> bool {
        if let Value::Function(other) = other {
            let this = &*self.function as *const parser::ast::Function;
            let other = &*other.function as *const parser::ast::Function;
            this == other
        } else {
            false
        }
    }
    fn plus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::String(string) => Ok(Value::String(format!("{}{}", self.to_string(), string))),
            _ => Err(()),
        }
    }

    fn call(
        &self,
        _start: usize,
        _end: usize,
        interpreter: &mut Interpreter,
        this: Value,
        mut args: Vec<Value>,
    ) -> Result<Value, Pos<String>> {
        interpreter.scope.push_scope(false);
        interpreter.scope.push_var("this".to_owned(), this, false);

        let arg_len = args.len().min(self.function.params.len());
        for (idx, arg) in args.drain(..arg_len).enumerate() {
            let param = self.function.params.get(idx).unwrap();
            interpreter
                .scope
                .push_var(param.value.0.clone(), arg, false);
        }

        let result = self.function.body.clone().eval(interpreter);
        interpreter.scope.pop_scope();
        Ok(result?.into_value())
    }
}

#[derive(Clone)]
pub struct NativeFunction(
    pub Rc<RefCell<dyn Fn(&mut Interpreter, Value, Vec<Value>) -> Result<Value, String>>>,
);

impl Object for NativeFunction {
    fn type_name(&self) -> &'static str {
        "native_function"
    }

    fn into_value(self) -> Value {
        Value::NativeFunction(self)
    }

    fn to_string(&self) -> String {
        "native_function(..){..}".to_owned()
    }

    fn call(
        &self,
        start: usize,
        end: usize,
        inter: &mut Interpreter,
        this: Value,
        args: Vec<Value>,
    ) -> Result<Value, Pos<String>> {
        let function = self.0.deref().borrow();
        function(inter, this, args).map_err(|x| Pos::new(start, end, x))
    }
}

impl Debug for NativeFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.to_string())
    }
}

impl NativeFunction {
    pub fn new(
        closure: impl Fn(&mut Interpreter, Value, Vec<Value>) -> Result<Value, String> + 'static,
    ) -> Self {
        NativeFunction(Rc::new(RefCell::new(closure)))
    }
}
