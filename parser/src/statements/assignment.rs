use crate::error::{Error, Severity, AddErrorCtx};
use crate::values::{Value, FieldAccess, Identifier};
use crate::{pop_expect, Parse, Parsed, impl_into_enum, first_value_of};
use lexer::{TokenValue};
use crate::statements::Statement;

#[derive(Debug)]
pub struct Assignment {
    pub target: Parsed<Value>,
    pub value: Parsed<Value>,
}

impl_into_enum!(Assignment => Statement:Assignment);
first_value_of!(
    LHS:
    Identifier,
    FieldAccess
);

impl Parse for Assignment {
    fn read<'a>(_pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        println!("parsing assignment..");
        let lhs = LHS::read(tokens[0].start, tokens)?.map(Value::from);
        let assign = pop_expect(lhs.end, tokens, TokenValue::Operator(Operator::Assign))
            .map_err(Severity::Recoverable)?;
        println!("--A");
        let rhs = Value::read(assign.end, tokens)
            .map_err(Severity::into_fatal)
            .ctx("parsing assignment")?;
        println!("--B");

        let semicolon = pop_expect(rhs.end, tokens, TokenValue::Semicolon)
            .map_err(Severity::Fatal)
            .ctx("parsing assignment")?;

        Ok(Parsed {
            start: lhs.start,
            end: semicolon.end,
            value: Assignment {
                target: lhs,
                value: rhs,
            },
        })
    }
}
