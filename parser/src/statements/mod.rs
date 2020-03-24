mod assignment;
mod r#return;
mod value_statement;

use crate::error::{Error, ParserResult, Severity};
use crate::values::Value;
use crate::{Parsed, ReadParse};
pub use assignment::*;
use lexer::{Token, TokenValue};
pub use r#return::*;
pub use value_statement::*;

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    Return(Return),
    ValueStatement(ValueStatement),
}

impl Statement {
    pub fn read(tokens: &mut &[Token]) -> ParserResult<Parsed<Statement>> {
        let first = &tokens[0];

        match Return::try_read(first.start, *tokens) {
            Ok((parsed, rest)) => {
                *tokens = rest;
                return Ok(parsed.map(Statement::Return));
            }
            Err(Severity::Fatal(err)) => return Err(err),
            _ => (),
        };

        match Assignment::try_read(first.start, *tokens) {
            Ok((parsed, rest)) => {
                *tokens = rest;
                return Ok(parsed.map(Statement::Assignment));
            }
            Err(Severity::Fatal(err)) => return Err(err),
            _ => (),
        };

        match ValueStatement::try_read(first.start, *tokens) {
            Ok((parsed, rest)) => {
                *tokens = rest;
                return Ok(parsed.map(Statement::ValueStatement));
            }
            Err(Severity::Recoverable(err)) => return Err(err),
            Err(Severity::Fatal(err)) => return Err(err),
        };
    }

    pub fn read_all(tokens: &mut &[Token]) -> ParserResult<Vec<Statement>> {
        let mut statements = vec![];
        while !tokens.is_empty() {
            statements.push(Statement::read(tokens)?.value);
        }
        Ok(statements)
    }
}
