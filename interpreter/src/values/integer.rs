use crate::values::{ConcreteObject, Dictionary, Object, Value};
use crate::Interpreter;

use std::f64::EPSILON;

impl Object for i64 {
    fn type_name(&self) -> &'static str {
        "integer"
    }

    fn into_value(self) -> Value {
        Value::Integer(self)
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn equal(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => self == int,
            Value::Float(float) => (*self as f64 - *float).abs() < EPSILON,
            _ => false,
        }
    }

    fn greater_than(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => self > int,
            Value::Float(float) => (*self as f64) > *float,
            _ => false,
        }
    }

    fn less_than(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => self < int,
            Value::Float(float) => (*self as f64) < *float,
            _ => false,
        }
    }

    fn plus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Integer(*self + *int)),
            Value::Float(float) => Ok(Value::Float(*self as f64 + *float)),
            Value::String(string) => Ok(Value::String(format!("{}{}", self, string))),
            _ => Err(()),
        }
    }

    fn minus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Integer(*self - *int)),
            Value::Float(float) => Ok(Value::Float(*self as f64 - *float)),
            _ => Err(()),
        }
    }

    fn multiply(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Integer(*self * *int)),
            Value::Float(float) => Ok(Value::Float(*self as f64 * *float)),
            _ => Err(()),
        }
    }

    fn divide(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Float(*self as f64 / *int as f64)),
            Value::Float(float) => Ok(Value::Float(*self as f64 / *float)),
            _ => Err(()),
        }
    }
}

impl ConcreteObject for i64 {
    fn type_name() -> &'static str {
        "integer"
    }

    fn convert_from(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(int) => Some(*int),
            Value::Float(float) if float.fract() == 0.0 => Some(*float as i64),
            _ => None,
        }
    }

    fn get_as(value: Value) -> Option<Self> {
        if let Value::Integer(int) = value {
            Some(int)
        } else {
            None
        }
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.integer_proto
    }
}
