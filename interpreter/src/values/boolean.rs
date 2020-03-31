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
}
