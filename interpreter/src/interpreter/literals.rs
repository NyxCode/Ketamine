use crate::values::{Array, ConcreteObject, Dictionary, Value};
use crate::{Eval, Evaluate, Interpreter};
use lexer::Pos;
use parser::ast::{List, Object, Range};

impl Evaluate for Pos<List> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: List(list),
        } = self;
        let mut elements = Vec::with_capacity(list.len());
        for element in list {
            let element = element
                .eval(interpreter)?
                .try_into_value()
                .map_err(|err| Pos::new(start, end, err))?;
            elements.push(element);
        }
        let array = Array::new(elements);
        Ok(Eval::Value(Value::Array(array)))
    }
}

impl Evaluate for Pos<Object> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Object(pairs),
        } = self;
        let object = Dictionary::default();
        for (k, v) in pairs {
            let value = v
                .eval(interpreter)?
                .try_into_value()
                .map_err(|err| Pos::new(start, end, err))?;
            if let Some(..) = object.insert(k.value.0, value) {
                let msg = "duplicate key in object literal".to_owned();
                return Err(Pos::new(start, end, msg));
            }
        }
        Ok(Eval::Value(Value::Dictionary(object)))
    }
}

impl Evaluate for Pos<Range> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Range { from, to },
        } = self;
        let from = from.eval(interpreter)?.into_value();
        let from = i64::try_convert_from(&from).map_err(|msg| Pos::new(start, end, msg))?;
        let to = to.eval(interpreter)?.into_value();
        let to = i64::try_convert_from(&to).map_err(|msg| Pos::new(start, end, msg))?;
        let array = (from..to).map(Value::Integer).collect::<Vec<_>>();
        Ok(Eval::Value(Value::Array(Array::new(array))))
    }
}
