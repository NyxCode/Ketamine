use crate::error::{Error, ErrorKind, Severity};
use crate::values::Value;
use crate::{pop_expect, Parse, Parsed};
use lexer::{Token, TokenValue};

#[derive(Debug)]
pub struct UnterminatedStatement(pub Value);

impl Parse for UnterminatedStatement {
    fn read(pos: usize, tokens: &mut &[Token]) -> Result<Parsed<Self>, Severity<Error>> {
        let statement = Value::read(pos, tokens).map_err(Severity::Recoverable)?;

        if let Some(token) = tokens.get(0) {
            let kind = ErrorKind::UnexpectedToken(token.value.clone());
            let err = Error::range(token.start, token.end, kind);
            return Err(Severity::Recoverable(err));
        }

        Ok(statement.map(UnterminatedStatement))
    }
}
