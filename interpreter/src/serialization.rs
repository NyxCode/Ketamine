use crate::values::Object;
use crate::{Array, Dictionary, Function, NativeFunction};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Serialize, Serializer};
use std::ops::Deref;

impl Serialize for Array {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let array = self.0.deref().borrow();
        let mut seq = serializer.serialize_seq(Some(array.len()))?;
        for e in array.iter() {
            seq.serialize_element(e)?;
        }
        seq.end()
    }
}

impl Serialize for Dictionary {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dict = self.0.deref().borrow();
        let mut map = serializer.serialize_map(Some(dict.len()))?;
        for (k, v) in dict.iter() {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl Serialize for Function {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl Serialize for NativeFunction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
