use crate::values::{Object, Value};

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

    fn equal(&self, other: &Value) -> bool {
        if let Value::Null = other {
            true
        } else {
            false
        }
    }
    fn plus(&self, other: &Value) -> Result<Value, ()> {
        match other {
            Value::String(string) => Ok(Value::String(format!("{}{}", self.to_string(), string))),
            _ => Err(()),
        }
    }
}
