use crate::values::{ConcreteObject, Dictionary, Object, Value};
use crate::Interpreter;

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
