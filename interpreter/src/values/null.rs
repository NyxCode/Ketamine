use crate::values::{Object, Value};
use parser::ast::BinaryOperator;

impl Object for () {
    fn type_name(&self) -> &'static str {
        "null"
    }

    fn into_value(self) -> Value {
        Value::Null
    }

    fn to_string(&self) -> String {
        "null".to_owned()
    }

    fn binary_op(&self, op: BinaryOperator, rhs: &Value) -> Result<Value, ()> {
        match op {
            BinaryOperator::Eq => match rhs {
                Value::Null => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            BinaryOperator::NotEq => match rhs {
                Value::Null => Ok(Value::Boolean(false)),
                _ => Ok(Value::Boolean(true)),
            },
            _ => Err(()),
        }
    }
}
