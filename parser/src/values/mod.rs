mod binary_operation;
mod function;
mod function_call;
mod identifier;
mod if_expr;
mod list;
mod object;
mod parentheses;
mod primitives;
mod unary_operation;

use crate::error::{Error, ErrorKind, ParserResult, Severity};
use crate::impl_into_value;
use crate::statements::Statement;
use crate::{find_closing_brace, first_value_of, pop_expect};
use crate::{Parsed, ReadParse};
pub use binary_operation::*;
pub use function::*;
pub use function_call::*;
pub use identifier::*;
pub use if_expr::*;
use lexer::{Token, TokenValue};
pub use list::*;
pub use object::*;
pub use parentheses::*;
pub use primitives::*;
use std::fmt::Debug;
use std::marker::PhantomData;
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
    IfExpr(IfExpr),
    //FieldAccess(FieldAccess),
    Function(Function),
    FunctionCall(FunctionCall),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Parentheses(Parentheses),
    Nothing,
}

impl_into_value!(BinaryOperation);
impl_into_value!(UnaryOperation);
impl_into_value!(Function);
impl_into_value!(IfExpr);
impl_into_value!(FunctionCall);
impl_into_value!(Object);
impl_into_value!(List);
impl_into_value!(String);
impl_into_value!(Identifier);
impl_into_value!(bool, Boolean);
impl_into_value!(i64, Integer);
impl_into_value!(f64, Float);

impl Value {
    pub fn read(pos: usize, tokens: &mut &[Token]) -> ParserResult<Parsed<Value>> {
        if tokens.is_empty() {
            return Err(Error::position(pos, ErrorKind::UnexpectedEOF));
        }
        let first = &tokens[0];
        let start = first.start;
        let end = first.end;

        first_value_of!(
            Values: BinaryOperation,
            UnaryOperation,
            Function,
            IfExpr,
            FunctionCall,
            Object,
            List,
            String,
            bool,
            i64,
            f64,
            Identifier,
            Parentheses,
        );
        return Values::read(pos, tokens)
            .map_err(Severity::into_inner)
            .map(|parsed| parsed.map(Into::into));
    }
}

fn read_code_block(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Vec<Statement>>, Error> {
    let open_brace = pop_expect(pos, tokens, TokenValue::BraceOpen)?;
    let close_brace_idx = find_closing_brace(open_brace.start, *tokens)?;
    let close_brace_pos = *&tokens[close_brace_idx].end;
    let mut body_tokens = &tokens[..close_brace_idx];
    let body = Statement::read_all(&mut body_tokens)?;
    *tokens = &tokens[(close_brace_idx + 1)..];
    Ok(Parsed {
        start: open_brace.start,
        end: close_brace_pos,
        value: body,
    })
}
