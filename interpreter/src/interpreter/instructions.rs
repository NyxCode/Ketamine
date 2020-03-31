use crate::values::Value;
use crate::{Eval, Evaluate, Interpreter};
use parser::ast::{Break, Return};
use parser::Pos;

impl Evaluate for Pos<Return> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Return(value),
        } = self;
        let value = match value {
            None => Value::Null,
            Some(value) => value
                .eval(interpreter)?
                .try_into_value()
                .map_err(|err| Pos::new(start, end, err))?,
        };
        return Ok(Eval::Return(value));
    }
}

impl Evaluate for Pos<Break> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Break(value),
        } = self;
        let value = match value {
            None => Value::Null,
            Some(value) => value
                .eval(interpreter)?
                .try_into_value()
                .map_err(|err| Pos::new(start, end, err))?,
        };
        return Ok(Eval::Break(value));
    }
}
