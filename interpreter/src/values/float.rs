use crate::values::{ConcreteObject, Dictionary, Object, Value};
use crate::Interpreter;
use std::f64::EPSILON;

impl Object for f64 {
    fn type_name(&self) -> &'static str {
        <Self as ConcreteObject>::type_name()
    }

    fn into_value(self) -> Value {
        Value::Float(self)
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn equal(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => (*self - *int as f64).abs() < EPSILON,
            Value::Float(float) => (*self - *float).abs() < EPSILON,
            _ => false,
        }
    }

    fn greater_than(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => *self > *int as f64,
            Value::Float(float) => self > float,
            _ => false,
        }
    }

    fn less_than(&self, other: &Value) -> bool {
        match other {
            Value::Integer(int) => *self < *int as f64,
            Value::Float(float) => self < float,
            _ => false,
        }
    }

    fn plus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Float(*self + *int as f64)),
            Value::Float(float) => Ok(Value::Float(*self + *float)),
            Value::String(string) => Ok(Value::String(format!("{}{}", self, string))),
            _ => Err(()),
        }
    }

    fn minus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Float(*self - *int as f64)),
            Value::Float(float) => Ok(Value::Float(*self - *float)),
            _ => Err(()),
        }
    }

    fn multiply(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Float(*self * *int as f64)),
            Value::Float(float) => Ok(Value::Float(*self * *float)),
            _ => Err(()),
        }
    }

    fn divide(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::Integer(int) => Ok(Value::Float(*self / *int as f64)),
            Value::Float(float) => Ok(Value::Float(*self / *float)),
            _ => Err(()),
        }
    }
}

impl ConcreteObject for f64 {
    fn type_name() -> &'static str {
        "float"
    }

    fn convert_from(value: &Value) -> Option<Self> {
        match value {
            Value::Integer(int) => Some(*int as f64),
            Value::Float(float) => Some(*float),
            _ => None,
        }
    }

    fn get_as(value: Value) -> Option<Self> {
        if let Value::Float(float) = value {
            Some(float)
        } else {
            None
        }
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.float_proto
    }
}
