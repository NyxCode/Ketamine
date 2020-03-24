use crate::values::Value;
use crate::{Identifier, Parsed, ParserResult};
use lexer::Token;

#[derive(Debug)]
pub struct FieldAccess {
    receiver: Box<Value>,
    field: Identifier,
}

impl FieldAccess {
    pub fn read(_pos: usize, _tokens: &mut &[Token]) -> ParserResult<Parsed<FieldAccess>> {
        unimplemented!()
    }
}
