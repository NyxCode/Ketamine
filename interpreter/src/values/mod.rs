mod array;
mod boolean;
mod dictionary;
mod float;
mod function;
mod integer;
mod null;
mod string;

pub use array::*;
pub use boolean::*;
pub use dictionary::*;
pub use float::*;
pub use function::*;
pub use integer::*;
pub use null::*;
pub use string::*;

use crate::scope::ScopeStack;
use crate::{Eval, Interpreter};
use parser::ast::{BinaryOperator, Ident};
use parser::Pos;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Array),
    Dictionary(Dictionary),
    Function(Function),
    NativeFunction(NativeFunction),
    Null,
}

impl Value {
    pub fn as_dyn(&self) -> &dyn Object {
        match self {
            Value::String(string) => string as &dyn Object,
            Value::Integer(int) => int as &dyn Object,
            Value::Float(float) => float as &dyn Object,
            Value::Boolean(bool) => bool as &dyn Object,
            Value::Array(array) => array as &dyn Object,
            Value::Dictionary(object) => object as &dyn Object,
            Value::Null => &() as &dyn Object,
            Value::Function(function) => function as &dyn Object,
            Value::NativeFunction(native) => native as &dyn Object,
        }
    }
}

pub trait ConcreteObject: Sized {
    fn type_name() -> &'static str;

    fn convert_from(value: &Value) -> Option<Self>;
    fn try_convert_from(value: &Value) -> Result<Self, String> {
        Self::convert_from(value)
            .ok_or_else(|| format!("can't cast {} to {}", value.type_name(), Self::type_name()))
    }

    fn get_as(value: Value) -> Option<Self>;
    fn try_get_as(value: Value) -> Result<Self, String> {
        Self::get_as(value)
            .ok_or_else(|| format!("mismatched type: expected {}", Self::type_name()))
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary;
}

pub trait Object {
    fn type_name(&self) -> &'static str;
    fn into_value(self) -> Value;
    fn to_string(&self) -> String;

    fn binary_op(&self, op: BinaryOperator, rhs: &Value) -> Result<Value, ()> {
        Err(())
    }
    fn call(
        &self,
        start: usize,
        end: usize,
        scope: &mut Interpreter,
        this: Value,
        args: Vec<Value>,
    ) -> Result<Value, Pos<String>> {
        let msg = format!("can't call a value of type {}", self.type_name());
        Err(Pos::new(start, end, msg))
    }
    fn get_index(&self, idx: &Value) -> Option<Value> {
        None
    }
    fn set_index(&self, idx: Value, val: Value) -> Result<(), String> {
        Err(format!("can't assign to index of {}", self.type_name()))
    }
    fn get_field(&self, field: &str) -> Option<Value> {
        None
    }
    fn set_field(&self, idx: Ident, val: Value) -> Result<(), String> {
        Err(format!(
            "can't assign to field {} of {}",
            idx.0,
            self.type_name()
        ))
    }
    fn iterator(&self) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        Err(format!("can't iterate over {}", self.type_name()))
    }
}

impl Object for Value {
    fn type_name(&self) -> &'static str {
        self.as_dyn().type_name()
    }

    fn into_value(self) -> Value {
        self
    }

    fn to_string(&self) -> String {
        self.as_dyn().to_string()
    }

    fn binary_op(&self, op: BinaryOperator, rhs: &Value) -> Result<Value, ()> {
        self.as_dyn().binary_op(op, rhs)
    }

    fn call(
        &self,
        start: usize,
        end: usize,
        scope: &mut Interpreter,
        this: Value,
        args: Vec<Value>,
    ) -> Result<Value, Pos<String>> {
        self.as_dyn().call(start, end, scope, this, args)
    }

    fn get_index(&self, idx: &Value) -> Option<Value> {
        self.as_dyn().get_index(idx)
    }

    fn set_index(&self, idx: Value, val: Value) -> Result<(), String> {
        self.as_dyn().set_index(idx, val)
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        self.as_dyn().get_field(field)
    }

    fn set_field(&self, idx: Ident, val: Value) -> Result<(), String> {
        self.as_dyn().set_field(idx, val)
    }

    fn iterator(&self) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        self.as_dyn().iterator()
    }
}
