use crate::values::{ConcreteObject, Dictionary, Object, Value};
use crate::Interpreter;
use std::cell::RefCell;
use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Array(pub Rc<RefCell<Vec<Value>>>);

impl Object for Array {
    fn type_name(&self) -> &'static str {
        <Self as ConcreteObject>::type_name()
    }

    fn into_value(self) -> Value {
        Value::Array(self)
    }

    fn to_string(&self) -> String {
        // TODO: optimize
        let elements = self
            .0
            .deref()
            .borrow()
            .iter()
            .map(Value::to_string)
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", elements)
    }

    fn get_index(&self, idx: &Value) -> Option<Value> {
        let idx = match idx {
            Value::Integer(idx) => *idx,
            Value::Float(idx) if idx.fract() == 0.0 => *idx as i64,
            _ => return None,
        };

        match TryInto::<usize>::try_into(idx) {
            Ok(idx) => Some(
                self.0
                    .deref()
                    .borrow()
                    .get(idx)
                    .cloned()
                    .unwrap_or(Value::Null),
            ),
            Err(..) => Some(Value::Null),
        }
    }

    fn set_index(&self, idx: Value, val: Value) -> Result<(), String> {
        let idx: usize = i64::try_get_as(idx)?
            .try_into()
            .map_err(|_| "index out of range".to_owned())?;
        let mut array = self.0.borrow_mut();
        if array.len() < idx {
            let increase = idx - array.len();
            array.extend_from_slice(&vec![Value::Null; increase]);
        }
        array.insert(idx, val);
        Ok(())
    }

    fn iterator(&self) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        let iter = ArrayIterator {
            array: self.clone(),
            pos: 0,
        };
        Ok(Box::new(iter))
    }
}

struct ArrayIterator {
    array: Array,
    pos: usize,
}

impl Iterator for ArrayIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.array.0.deref().borrow().get(self.pos).cloned()?;
        self.pos += 1;
        Some(element)
    }
}

impl Array {
    pub fn new(array: Vec<Value>) -> Self {
        Array(Rc::new(RefCell::new(array)))
    }
}

impl ConcreteObject for Array {
    fn type_name() -> &'static str {
        "array"
    }

    fn convert_from(value: &Value) -> Option<Self> {
        match value {
            Value::Array(array) => Some(array.clone()),
            _ => None,
        }
    }

    fn get_as(value: Value) -> Option<Self> {
        match value {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.array_proto
    }
}
