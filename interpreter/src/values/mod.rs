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

use crate::Interpreter;
use parser::ast::{Ident};
use parser::Pos;

#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
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

    fn equal(&self, _other: &Value) -> bool {
        false
    }
    fn greater_than(&self, _other: &Value) -> bool {
        false
    }
    fn less_than(&self, _other: &Value) -> bool {
        false
    }

    fn plus(&self, _other: &Value) -> Result<Value, ()> {
        Err(())
    }
    fn minus(&self, _other: &Value) -> Result<Value, ()> {
        Err(())
    }
    fn multiply(&self, _other: &Value) -> Result<Value, ()> {
        Err(())
    }
    fn divide(&self, _other: &Value) -> Result<Value, ()> {
        Err(())
    }

    fn call(
        &self,
        start: usize,
        end: usize,
        _scope: &mut Interpreter,
        _this: Value,
        _args: Vec<Value>,
    ) -> Result<Value, Pos<String>> {
        let msg = format!("can't call a value of type {}", self.type_name());
        Err(Pos::new(start, end, msg))
    }
    fn get_index(&self, _idx: &Value) -> Option<Value> {
        None
    }
    fn set_index(&self, _idx: Value, _val: Value) -> Result<(), String> {
        Err(format!("can't assign to index of {}", self.type_name()))
    }
    fn get_field(&self, _field: &str) -> Option<Value> {
        None
    }
    fn set_field(&self, idx: Ident, _val: Value) -> Result<(), String> {
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

    fn equal(&self, other: &Value) -> bool {
        self.as_dyn().equal(other)
    }

    fn greater_than(&self, other: &Value) -> bool {
        self.as_dyn().greater_than(other)
    }

    fn less_than(&self, other: &Value) -> bool {
        self.as_dyn().less_than(other)
    }

    fn plus(&self, other: &Value) -> Result<Value, ()> {
        self.as_dyn().plus(other)
    }

    fn minus(&self, other: &Value) -> Result<Value, ()> {
        self.as_dyn().minus(other)
    }

    fn multiply(&self, other: &Value) -> Result<Value, ()> {
        self.as_dyn().multiply(other)
    }
    fn divide(&self, other: &Value) -> Result<Value, ()> {
        self.as_dyn().divide(other)
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}
