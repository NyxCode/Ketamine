use crate::{find_closing_delimiter, parse_list, Parse, Token};
use lexer::{Pos, TokenValue};

mod assignment;
mod call;
mod field_access;
mod flow;
mod function;
mod ident;
mod r#if;
mod index;
mod literals;
mod loops;
mod operation;
mod parentheses;
mod primitives;
mod range;
mod statement;

pub use assignment::*;
pub use call::*;
pub use field_access::*;
pub use flow::*;
pub use function::*;
pub use ident::*;
pub use index::*;
pub use literals::*;
pub use loops::*;
pub use operation::*;
pub use parentheses::*;
pub use primitives::*;
pub use r#if::*;
pub use range::*;
pub use statement::*;

use crate::error::{Error, ParseResult, ResultExt};
use crate::first_value_of;
use crate::token_ext::TokenExt;

use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub enum AST {
    Ident(Ident),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Break(Break),
    Continue(Continue),
    Return(Return),
    Assignment(Assignment),
    FieldAccess(FieldAccess),
    Index(Index),
    Function(Function),
    If(If),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Parentheses(Parentheses),
    Call(Call),
    List(List),
    Object(Object),
    Range(Range),
    WhileLoop(WhileLoop),
    ForLoop(ForLoop),
}

first_value_of!(
    AtomicValues: Return,
    Break,
    Continue,
    Function,
    If,
    ForLoop,
    WhileLoop,
    UnaryOperation,
    Parentheses,
    List,
    Object,
    Ident,
    Primitives,
);

impl Parse for AST {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut ast: Pos<AST> = AST::parse_atomic(pos, tokens)?;

        loop {
            match tokens.peek(pos).ok() {
                None
                | Some(Pos {
                    value: TokenValue::Semicolon,
                    ..
                })
                | Some(Pos {
                    value: TokenValue::Comma,
                    ..
                })
                | Some(Pos {
                    value: TokenValue::ParenthesesClose,
                    ..
                })
                | Some(Pos {
                    value: TokenValue::BraceClose,
                    ..
                })
                | Some(Pos {
                    value: TokenValue::BracketClose,
                    ..
                }) => return Ok(ast),
                Some(_next) => ast = AST::append(ast, tokens)?,
            };
        }
    }
}

impl AST {
    pub fn parse_atomic<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        Ok(AtomicValues::parse(pos, tokens)?.map(AST::from))
    }

    fn append<'a>(prev: Pos<Self>, tokens: &mut &'a [Token]) -> ParseResult<'a, AST> {
        let next = tokens.peek_unwrap();
        match &next.value {
            TokenValue::Assign => {
                tokens.pop_unwrap();
                let next = AST::parse(prev.end, tokens)?;
                Ok(Pos {
                    start: prev.start,
                    end: next.end,
                    value: AST::Assignment(Assignment {
                        receiver: prev.map(Box::new),
                        value: next.map(Box::new),
                    }),
                })
            }
            TokenValue::Dot => {
                tokens.pop_unwrap();
                let field = Ident::parse(prev.end, tokens)?;
                Ok(Pos {
                    start: prev.start,
                    end: field.end,
                    value: AST::FieldAccess(FieldAccess {
                        value: prev.map(Box::new),
                        field,
                    }),
                })
            }
            TokenValue::BracketOpen => {
                let open = tokens.pop_unwrap();
                let index = AST::parse(open.end, tokens)?.map(Box::new);
                let close = tokens.pop_expect(index.end, &TokenValue::BracketClose)?;
                Ok(Pos {
                    start: prev.start,
                    end: close.end,
                    value: AST::Index(Index {
                        value: prev.map(Box::new),
                        index,
                    }),
                })
            }
            TokenValue::ParenthesesOpen => {
                let args: Pos<Vec<Pos<AST>>> = parse_list(
                    next.start,
                    tokens,
                    TokenValue::ParenthesesOpen,
                    TokenValue::ParenthesesClose,
                    TokenValue::Comma,
                )
                .into_fatal()?;
                Ok(Pos {
                    start: prev.start,
                    end: args.end,
                    value: AST::Call(Call {
                        value: prev.map(Box::new),
                        args: args.value,
                    }),
                })
            }
            TokenValue::Range => {
                let range_token = tokens.pop_unwrap();
                let to = AST::parse_atomic(range_token.end, tokens).into_fatal()?;
                Ok(Pos {
                    start: prev.start,
                    end: to.end,
                    value: AST::Range(Range {
                        from: prev.map(Box::new),
                        to: to.map(Box::new),
                    }),
                })
            }
            // i want a if-let in match guards!
            operator if BinaryOperator::try_from(operator).is_ok() => {
                let op_token = tokens.pop_unwrap();
                let op = BinaryOperator::try_from(operator).unwrap();
                let op = Pos::new(op_token.start, op_token.end, op);
                let next_val = AST::parse(op_token.end, tokens).into_fatal()?;
                Ok(BinaryOperation::from_left_to_right(prev, op, next_val)
                    .map(AST::BinaryOperation))
            }
            other => Err(Pos {
                start: next.start,
                end: next.end,
                value: Error::Unexpected {
                    unexpected: other,
                    expected: TokenValue::Semicolon.name(),
                }
                .fatal(),
            }),
        }
    }
}

pub fn parse_delimited_block<'a>(
    pos: usize,
    tokens: &mut &'a [Token],
) -> ParseResult<'a, Vec<Pos<Statement>>> {
    let brace_open = tokens
        .pop_expect(pos, &TokenValue::BraceOpen)
        .into_recoverable()?;
    let brace_close_idx = find_closing_delimiter(
        brace_open.start,
        tokens,
        &TokenValue::BraceOpen,
        &TokenValue::BraceClose,
        1,
    )
    .into_fatal()?;

    let mut code_tokens = &tokens[..brace_close_idx];
    let code = CodeBlock::parse(pos, &mut code_tokens).into_fatal()?;
    assert!(code_tokens.is_empty());

    *tokens = &tokens[brace_close_idx..];
    let brace_close = tokens
        .pop_expect(code.end, &TokenValue::BraceClose)
        .into_fatal()?;
    Ok(Pos {
        start: brace_open.start,
        end: brace_close.end,
        value: code.value,
    })
}
