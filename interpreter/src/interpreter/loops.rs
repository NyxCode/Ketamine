use crate::values::{Object, Value};
use crate::{Eval, Evaluate, Interpreter};
use lexer::Pos;
use parser::ast::{ForLoop, WhileLoop};

impl Evaluate for Pos<ForLoop> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value:
                ForLoop {
                    binding,
                    iterator,
                    body,
                },
        } = self;

        let iterator = match iterator.eval(interpreter)? {
            Eval::Value(val) => val,
            other => return Ok(other),
        }
        .iterator()
        .map_err(|err| Pos::new(start, end, err))?;

        interpreter.scope(true, |interpreter| {
            for element in iterator {
                interpreter.scope.push_var(&binding.value.0, element, true);
                match body.clone().eval(interpreter)? {
                    ret @ Eval::Return(..) => return Ok(ret),
                    Eval::Break(val) => return Ok(Eval::Value(val)),
                    Eval::Continue => continue,
                    _ => (),
                };
            }
            Ok(Eval::Value(Value::Null))
        })
    }
}

impl Evaluate for Pos<WhileLoop> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            value: WhileLoop { condition, body },
            ..
        } = self;

        while let Value::Boolean(true) = condition.clone().eval(interpreter)?.into_value() {
            match body.clone().eval(interpreter)? {
                ret @ Eval::Return(..) => return Ok(ret),
                Eval::Break(val) => return Ok(Eval::Value(val)),
                Eval::Continue => continue,
                _ => (),
            };
        }
        Ok(Eval::Value(Value::Null))
    }
}
