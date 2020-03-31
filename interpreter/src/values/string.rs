use crate::values::{Object, Value};
use parser::ast::BinaryOperator;

impl Object for String {
    fn type_name(&self) -> &'static str {
        "string"
    }

    fn into_value(self) -> Value {
        Value::String(self)
    }

    fn to_string(&self) -> String {
        self.clone()
    }

    fn binary_op(&self, op: BinaryOperator, rhs: &Value) -> Result<Value, ()> {
        match op {
            BinaryOperator::Add => Ok(Value::String(format!("{}{}", self, rhs.to_string()))),
            _ => Err(()),
        }
    }

    fn iterator(&self) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        let iter = self
            .chars()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|c| Value::String(c.to_string()));
        Ok(Box::new(iter))
    }
}
