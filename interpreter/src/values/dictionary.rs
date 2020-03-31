use crate::values::{ConcreteObject, Object, Value};
use crate::Interpreter;
use parser::ast::Ident;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Dictionary(pub Rc<RefCell<HashMap<String, Value>>>);

impl Object for Dictionary {
    fn type_name(&self) -> &'static str {
        "object"
    }

    fn into_value(self) -> Value {
        Value::Dictionary(self)
    }

    fn to_string(&self) -> String {
        // TODO: optimize
        let pairs = self
            .0
            .deref()
            .borrow()
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.to_string()))
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{{}}}", pairs)
    }

    fn get_field(&self, field: &str) -> Option<Value> {
        Some(
            self.0
                .deref()
                .borrow()
                .get(field)
                .cloned()
                .unwrap_or(Value::Null),
        )
    }

    fn set_field(&self, idx: Ident, val: Value) -> Result<(), String> {
        self.0.deref().borrow_mut().insert(idx.0, val);
        Ok(())
    }

    fn iterator(&self) -> Result<Box<dyn Iterator<Item = Value>>, String> {
        let iter = DictionaryIter {
            dict: self.clone(),
            pos: 0,
        };
        Ok(Box::new(iter))
    }
}

struct DictionaryIter {
    dict: Dictionary,
    pos: usize,
}

impl Iterator for DictionaryIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self
            .dict
            .0
            .deref()
            .borrow()
            .iter()
            .skip(self.pos)
            .map(|(k, v)| {
                let dict = Dictionary::default();
                dict.insert("key".to_owned(), Value::String(k.clone()));
                dict.insert("value".to_owned(), v.to_owned());
                Value::Dictionary(dict)
            })
            .next()?;
        self.pos += 1;
        Some(next)
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary(Rc::new(RefCell::new(HashMap::new())))
    }
}

impl ConcreteObject for Dictionary {
    fn type_name() -> &'static str {
        "object"
    }

    fn convert_from(value: &Value) -> Option<Self> {
        match value {
            Value::Dictionary(dic) => Some(dic.clone()),
            _ => None,
        }
    }

    fn get_as(value: Value) -> Option<Self> {
        if let Value::Dictionary(dict) = value {
            Some(dict)
        } else {
            None
        }
    }

    fn get_prototype(interpreter: &Interpreter) -> &Dictionary {
        &interpreter.object_proto
    }
}

impl Dictionary {
    pub fn insert(&self, key: String, value: Value) -> Option<Value> {
        self.0.deref().borrow_mut().insert(key, value)
    }
}
