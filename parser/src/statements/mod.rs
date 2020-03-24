mod assignment;
mod terminated;
mod unterminated;

use crate::error::{Error, ParserResult, Severity};
use crate::values::{ForExpr, IfExpr, Loop, Value, While};
use crate::{first_value_of, impl_into_enum};
use crate::{Parse, Parsed};
pub use assignment::*;
use lexer::{Token, TokenValue};
pub use terminated::*;
pub use unterminated::*;

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    IfExpr(IfExpr),
    ForExpr(ForExpr),
    While(While),
    Loop(Loop),
    TerminatedStatement(TerminatedStatement),
    UnterminatedStatement(UnterminatedStatement),
}

impl_into_enum!(Assignment, Statement);
impl_into_enum!(IfExpr, Statement);
impl_into_enum!(TerminatedStatement, Statement);
impl_into_enum!(UnterminatedStatement, Statement);

impl Statement {
    pub fn read(tokens: &mut &[Token]) -> ParserResult<Parsed<Statement>> {
        let first = &tokens[0];

        first_value_of!(
            Statements: IfExpr,
            ForExpr,
            While,
            Loop,
            Assignment,
            UnterminatedStatement,
            TerminatedStatement
        );

        return return Statements::read(first.start, tokens)
            .map_err(Severity::into_inner)
            .map(|parsed| parsed.map(Into::into));
    }

    pub fn read_all(tokens: &mut &[Token]) -> ParserResult<Vec<Statement>> {
        let mut statements = vec![];
        while !tokens.is_empty() {
            statements.push(Statement::read(tokens)?.value);
        }
        Ok(statements)
    }
}
