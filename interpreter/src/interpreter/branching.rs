use crate::values::{Object, Value};
use crate::{Eval, Evaluate, Interpreter};
use lexer::Pos;
use parser::ast::{If, IfBranch};

impl Evaluate for Pos<If> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value:
                If {
                    if_branch,
                    else_if_branches,
                    else_branch,
                },
        } = self;

        if let Some(res) = evaluate_if(if_branch, interpreter)? {
            return Ok(res);
        }
        for if_else in else_if_branches {
            if let Some(res) = evaluate_if(if_else, interpreter)? {
                return Ok(res);
            }
        }
        let else_branch = if let Some(else_branch) = else_branch {
            else_branch
                .eval(interpreter)?
                .try_into_value()
                .map_err(|err| Pos::new(start, end, err))?
        } else {
            Value::Null
        };
        Ok(Eval::Value(else_branch))
    }
}

fn evaluate_if(
    branch: Pos<IfBranch>,
    scope: &mut Interpreter,
) -> Result<Option<Eval>, Pos<String>> {
    let Pos {
        value: IfBranch { condition, body },
        ..
    } = branch;

    let cond_start = condition.start;
    let cond_end = condition.end;
    let cond = condition
        .eval(scope)?
        .try_into_value()
        .map_err(|err| Pos::new(cond_start, cond_end, err))?;

    let condition = match cond {
        Value::Boolean(cond) => cond,
        other => {
            let msg = format!("expected boolean, got {}", other.type_name());
            return Err(Pos::new(cond_start, cond_end, msg));
        }
    };
    if condition {
        let result = body.eval(scope)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}
