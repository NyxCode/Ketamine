use crate::values::{ConcreteObject, Dictionary, Object, Value};
use crate::Interpreter;
use parser::ast::BinaryOperator;
use std::str::FromStr;

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

    fn binary_op(&self, op: BinaryOperator, rhs: &Value) -> Result<Value, ()> {
        match op {
            BinaryOperator::Add => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(*self + *rhs)),
                Value::Float(rhs) => Ok(Value::Float(*self as f64 + *rhs)),
                Value::String(string) => Ok(Value::String(format!("{}{}", self, string))),
                _ => Err(()),
            },
            BinaryOperator::Sub => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(*self - *rhs)),
                Value::Float(rhs) => Ok(Value::Float(*self as f64 - *rhs)),
                _ => Err(()),
            },
            BinaryOperator::Mul => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(*self * *rhs)),
                Value::Float(rhs) => Ok(Value::Float(*self as f64 * *rhs)),
                _ => Err(()),
            },
            BinaryOperator::Div => match rhs {
                Value::Integer(rhs) => Ok(Value::Integer(*self / *rhs)),
                Value::Float(rhs) => Ok(Value::Float(*self as f64 / *rhs)),
                _ => Err(()),
            },
            BinaryOperator::Eq => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self == *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean(*self as f64 == *rhs)),
                _ => Ok(Value::Boolean(false)),
            },
            BinaryOperator::NotEq => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self != *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean(*self as f64 != *rhs)),
                _ => Ok(Value::Boolean(false)),
            },
            BinaryOperator::GreaterThan => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self > *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean(*self as f64 > *rhs)),
                _ => Err(()),
            },
            BinaryOperator::LessThan => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self < *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean((*self as f64) < *rhs)),
                _ => Err(()),
            },
            BinaryOperator::GreaterEqThan => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self >= *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean(*self as f64 >= *rhs)),
                _ => Err(()),
            },
            BinaryOperator::LessEqThan => match rhs {
                Value::Integer(rhs) => Ok(Value::Boolean(*self <= *rhs)),
                Value::Float(rhs) => Ok(Value::Boolean(*self as f64 <= *rhs)),
                _ => Err(()),
            },
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
