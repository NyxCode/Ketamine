use crate::values::{Object, Value};
use crate::{Dictionary, HasPrototype, HasTypeName, Interpreter, ObjectConversion};
use std::convert::TryInto;
use std::ops::Deref;

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

    fn get_index(&self, idx: &Value) -> Option<Value> {
        match idx {
            Value::Integer(int) => {
                let index: usize = match (*int).try_into() {
                    Ok(index) => index,
                    Err(..) => return Some(Value::Null),
                };
                let value = self
                    .chars()
                    .nth(index)
                    .map(|c| Value::String(c.to_string()))
                    .unwrap_or(Value::Null);
                Some(value)
            }
            Value::Array(array) => {
                let array = array.0.deref().borrow();
                let mut out = String::with_capacity(array.len());
                for element in array.iter() {
                    if let Value::Integer(idx) = element {
                        if let Ok(idx) = (*idx).try_into() {
                            if let Some(c) = self.chars().nth(idx) {
                                out.push(c)
                            }
                        }
                    }
                }
                Some(Value::String(out))
            }
            _ => None,
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

impl HasPrototype for String {
    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.string_proto
    }
}

impl HasTypeName for String {
    fn type_name() -> &'static str {
        "string"
    }
}

impl ObjectConversion for String {
    fn get_as(value: Value) -> Option<Self> {
        match value {
            Value::String(string) => Some(string),
            _ => None,
        }
    }

    fn convert_from(value: &Value) -> Option<Self> {
        Some(value.to_string())
    }
}
