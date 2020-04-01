use crate::values::{Object, Value};

impl Object for bool {
    fn type_name(&self) -> &'static str {
        "boolean"
    }

    fn into_value(self) -> Value {
        Value::Boolean(self)
    }

    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn equal(&self, other: &Value) -> bool {
        if let Value::Boolean(other) = other {
            other == self
        } else {
            false
        }
    }

    fn plus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::String(string) => Ok(Value::String(format!("{}{}", self, string))),
            _ => Err(()),
        }
    }
}
