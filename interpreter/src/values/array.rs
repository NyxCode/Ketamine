use std::cell::RefCell;
use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;

use crate::values::{Dictionary, HasPrototype, Object, Value};
use crate::{HasTypeName, Interpreter, ObjectConversion};

#[derive(Debug, Clone)]
pub struct Array(pub Rc<RefCell<Vec<Value>>>);

impl Object for Array {
    fn type_name(&self) -> &'static str {
        <Self as HasTypeName>::type_name()
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

    fn equal(&self, other: &Value) -> bool {
        if let Value::Array(arr) = other {
            *arr.0.deref().borrow() == *self.0.deref().borrow()
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

    fn get_index(&self, idx: &Value) -> Option<Value> {
        let this = self.0.deref().borrow();
        match idx {
            Value::Integer(idx) => match (*idx).try_into() {
                Ok(index) => Some(this.get::<usize>(index).cloned().unwrap_or(Value::Null)),
                Err(..) => Some(Value::Null),
            },
            Value::Array(array) => {
                let array = array.0.deref().borrow();

                let sub_array = array
                    .iter()
                    .flat_map(|element| i64::get_as(element.clone()))
                    .flat_map(|idx| TryInto::<usize>::try_into(idx).ok())
                    .flat_map(|idx| this.get(idx))
                    .cloned()
                    .collect::<Vec<_>>();

                Some(Value::Array(Array::new(sub_array)))
            }
            _ => None,
        }
    }

    fn set_index(&self, idx: Value, val: Value) -> Result<(), String> {
        let idx: usize = i64::try_get_as(idx)?
            .try_into()
            .map_err(|_| "index out of range".to_owned())?;
        let mut array = self.0.borrow_mut();
        if array.len() <= idx {
            let increase = idx - array.len() + 1;
            array.extend_from_slice(&vec![Value::Null; increase]);
        }
        std::mem::replace(&mut array[idx], val);
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

impl HasPrototype for Array {
    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.array_proto
    }
}

impl HasTypeName for Array {
    fn type_name() -> &'static str {
        "array"
    }
}

impl ObjectConversion for Array {
    fn get_as(value: Value) -> Option<Self> {
        match value {
            Value::Array(array) => Some(array),
            _ => None,
        }
    }

    fn convert_from(value: &Value) -> Option<Self> {
        match value {
            Value::Array(array) => Some(array.clone()),
            _ => None,
        }
    }
}
