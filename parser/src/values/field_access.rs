use crate::values::{Value, Object, Call, List, Parentheses, BinaryOperation};
use crate::{Identifier, Parsed, ParserResult, impl_into_enum, first_value_of, Parse, pop_expect};
use lexer::{Token, TokenValue};
use crate::error::Severity;

#[derive(Debug)]
pub struct FieldAccess {
    pub receiver: Box<Parsed<Value>>,
    pub fields: Vec<Parsed<Identifier>>,
}

impl_into_enum!(FieldAccess => Value:FieldAccess);

impl Parse for FieldAccess {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        first_value_of!(
            LHS:
            Identifier,
            Object,
            Call,
            List,
            String,
            i64,
            f64,
            bool,
            Parentheses,
            BinaryOperation
        );

        let lhs = LHS::read(pos, tokens)?.map(Value::from);
        let start = lhs.start;
        let dot = pop_expect(lhs.end, tokens, TokenValue::Dot).map_err(Severity::Recoverable)?;
        let first = Identifier::read(dot.end, tokens).map_err(Severity::into_fatal)?;

        let mut field_access = FieldAccess { receiver: Box::new(lhs), fields: vec![first] };
        loop {
            match DotAndIdent::try_read(pos, tokens) {
                Err(fatal @ Severity::Fatal(..)) => return Err(fatal),
                Err(..) => break,
                Ok((next, rest)) => {
                    *tokens = rest;
                    field_access.fields.push(next.map(|next| next.0.value))
                }
            }
        }

        Ok(Parsed {
            start,
            end: field_access.fields.last().unwrap().end,
            value: field_access,
        })
    }
}

#[derive(Debug)]
struct DotAndIdent(Parsed<Identifier>);

impl Parse for DotAndIdent {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let dot = pop_expect(pos, tokens, TokenValue::Dot)
            .map_err(Severity::Recoverable)?;
        let rhs = Identifier::read(pos, tokens)
            .map_err(Severity::into_fatal)?;
        Ok(Parsed {
            start: dot.start,
            end: rhs.end,
            value: DotAndIdent(rhs),
        })
    }
}