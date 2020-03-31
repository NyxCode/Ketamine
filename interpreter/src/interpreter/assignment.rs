use crate::values::{Object, Value};
use crate::{Eval, Evaluate, Interpreter};
use parser::ast::{Assignment, AST};
use parser::Pos;

impl Evaluate for Pos<Assignment> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Assignment { receiver, value },
        } = self;

        let value = value.eval(interpreter)?.into_value();

        match *receiver.value {
            AST::Ident(ident) => {
                interpreter.scope.push_var(ident.0, value, false);
            }
            AST::FieldAccess(access) => {
                let object = access
                    .value
                    .eval(interpreter)?
                    .try_into_value()
                    .map_err(|err| Pos::new(start, end, err))?;
                object
                    .set_field(access.field.value, value)
                    .map_err(|err| Pos::new(start, end, err))?;
            }
            AST::Index(index) => {
                let idx_val = index
                    .value
                    .eval(interpreter)?
                    .try_into_value()
                    .map_err(|err| Pos::new(start, end, err))?;
                let idx_idx = index
                    .index
                    .eval(interpreter)?
                    .try_into_value()
                    .map_err(|err| Pos::new(start, end, err))?;
                idx_val
                    .set_index(idx_idx, value)
                    .map_err(|err| Pos::new(start, end, err))?;
            }
            other => panic!("{:?}", other),
        };
        Ok(Eval::Value(Value::Null))
    }
}
