use crate::values::{Object, Value};
use crate::{ConcreteObject, Dictionary, Interpreter};


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

    fn equal(&self, other: &Value) -> bool {
        if let Value::String(string) = other {
            string == self
        } else {
            false
        }
    }

    fn plus(&self, other: &Value) -> Result<Value, ()> {
        Ok(Value::String(format!("{}{}", self, other.to_string())))
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

impl ConcreteObject for String {
    fn type_name() -> &'static str {
        "string"
    }

    fn convert_from(value: &Value) -> Option<Self> {
        Some(value.to_string())
    }

    fn get_as(value: Value) -> Option<Self> {
        match value {
            Value::String(string) => Some(string),
            _ => None
        }
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.string_proto
    }
}