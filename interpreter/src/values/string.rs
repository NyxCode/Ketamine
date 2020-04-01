use crate::values::{Object, Value};


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
