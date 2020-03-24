use crate::error::{Error, ErrorKind, Severity};
use crate::values::Value;
use crate::{Parse, Parsed};
use lexer::{Token};

#[derive(Debug)]
pub struct UnterminatedStatement(pub Value);

impl Parse for UnterminatedStatement {
    fn read<'a>(pos: usize, tokens: &mut &'a [Token]) -> Result<Parsed<Self>, Severity<'a>> {
        let statement = Value::read(pos, tokens)?;

        if let Some(token) = tokens.get(0) {
            let kind = ErrorKind::UnexpectedToken(&token.value);
            let err = Error::range(token.start, token.end, kind);
            return Err(Severity::Recoverable(err));
        }

        Ok(statement.map(UnterminatedStatement))
    }
}
