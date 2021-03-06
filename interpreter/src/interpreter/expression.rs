use crate::values::{Object, Value};
use crate::{Eval, Evaluate, Interpreter};
use lexer::Pos;
use parser::ast::{BinaryOperation, BinaryOperator, UnaryOperation, UnaryOperator};

impl Evaluate for Pos<BinaryOperation> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: BinaryOperation { lhs, op, rhs },
        } = self;

        let operator = op.value;
        let lhs = lhs
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;
        let rhs = rhs
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;

        let result = match &operator {
            BinaryOperator::Add => lhs.plus(&rhs),
            BinaryOperator::Sub => lhs.minus(&rhs),
            BinaryOperator::Mul => lhs.multiply(&rhs),
            BinaryOperator::Div => lhs.divide(&rhs),
            BinaryOperator::Eq => Ok(Value::Boolean(lhs.equal(&rhs))),
            BinaryOperator::NotEq => Ok(Value::Boolean(!lhs.equal(&rhs))),
            BinaryOperator::GreaterThan => Ok(Value::Boolean(lhs.greater_than(&rhs))),
            BinaryOperator::LessThan => Ok(Value::Boolean(lhs.less_than(&rhs))),
            BinaryOperator::GreaterEqThan => Ok(Value::Boolean(!lhs.less_than(&rhs))),
            BinaryOperator::LessEqThan => Ok(Value::Boolean(!lhs.greater_than(&rhs))),
        };

        let result = result.map_err(|_| {
            let (v1, v2) = operator.verb();
            let msg = format!(
                "can't {} {} {} {}",
                v1,
                rhs.type_name(),
                v2,
                lhs.type_name()
            );
            Pos {
                start,
                end,
                value: msg,
            }
        })?;

        Ok(Eval::Value(result))
    }
}

impl Evaluate for Pos<UnaryOperation> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: UnaryOperation { op, value },
        } = self;

        let value = value
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;

        match op.value {
            UnaryOperator::Minus => match value {
                Value::Integer(int) => Ok(Eval::Value(Value::Integer(-int))),
                Value::Float(float) => Ok(Eval::Value(Value::Float(-float))),
                _ => {
                    let msg = format!("can't apply unary minus to {}", value.type_name());
                    Err(Pos::new(start, end, msg))
                }
            },
            UnaryOperator::Negate => match value {
                Value::Boolean(boolean) => Ok(Eval::Value(Value::Boolean(!boolean))),
                _ => {
                    let msg = format!("can't negate {}", value.type_name());
                    Err(Pos::new(start, end, msg))
                }
            },
        }
    }
}
