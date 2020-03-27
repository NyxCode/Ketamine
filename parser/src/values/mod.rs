mod binary_operation;
mod r#break;
mod r#continue;
mod r#for;
mod function;
mod call;
mod identifier;
mod r#if;
mod list;
mod r#loop;
mod object;
mod parentheses;
mod primitives;
mod range;
mod r#return;
mod unary_operation;
mod r#while;
mod field_access;
mod expression;

use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::impl_into_enum;
use crate::statements::Statement;
use crate::{find_closing_brace, first_value_of, pop_expect};
use crate::{Parse, Parsed};
pub use binary_operation::*;
pub use r#for::*;
pub use function::*;
pub use call::*;
pub use identifier::*;
pub use r#if::*;
use lexer::{Token, TokenValue};
pub use list::*;
pub use object::*;
pub use parentheses::*;
pub use primitives::*;
pub use r#break::*;
pub use r#continue::*;
pub use r#loop::*;
pub use r#return::*;
pub use r#while::*;
pub use range::*;
pub use field_access::*;
use std::fmt::Debug;

pub use unary_operation::*;

#[derive(Debug)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Object(Object),
    List(List),
    Identifier(Identifier),
    FieldAccess(FieldAccess),
    Range(Range),
    If(If),
    For(For),
    While(While),
    Loop(Loop),
    Break(Break),
    Return(Return),
    Continue(Continue),
    //FieldAccess(FieldAccess),
    Function(Function),
    Call(Call),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Parentheses(Parentheses),
    Nothing,
}

pub type ParsedValue = Parsed<Value>;

impl_into_enum!(BinaryOperation => Value:BinaryOperation);
impl_into_enum!(Function => Value:Function);
impl_into_enum!(Object => Value:Object);
impl_into_enum!(List => Value:List);
impl_into_enum!(Identifier => Value:Identifier);

impl Parse for Value {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        if tokens.is_empty() {
            let kind = ErrorKind::UnexpectedEOF;
            let err = Error::position(pos, kind);
            return Err(Severity::Recoverable(err));
        }

        first_value_of!(
            Values: Return,
            Continue,
            Break,
            //BinaryOperation,
            //Range,
            UnaryOperation,
            Function,
            If,
            For,
            While,
            Loop,
            Call,
            Object,
            List,
            String,
            bool,
            i64,
            f64,
            FieldAccess,
            Identifier,
            Parentheses,
        );
        let value = Values::read(pos, tokens)?
            .map(Into::into);
        Ok(value)
    }
}

fn read_code_block<'a >(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Vec<Parsed<Statement>>>, Error<'a>> {
    let open_brace = pop_expect(pos, tokens, TokenValue::BraceOpen)?;
    let close_brace_idx = find_closing_brace(open_brace.start, *tokens)?;
    let close_brace_pos = tokens[close_brace_idx].end;
    let mut body_tokens = &tokens[..close_brace_idx];
    let body = Statement::read_all(pos, &mut body_tokens)?;
    *tokens = &tokens[(close_brace_idx + 1)..];
    Ok(Parsed {
        start: open_brace.start,
        end: close_brace_pos,
        value: body,
    })
}
