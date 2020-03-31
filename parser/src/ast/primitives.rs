use crate::ast::AST;
use crate::error::{Error, ParseResult};
use crate::impl_into_enum;
use crate::token_ext::TokenExt;
use crate::{Parse, Pos, Token};
use lexer::TokenValue;

#[derive(Debug)]
pub struct Primitives(pub AST);

impl From<Primitives> for AST {
    fn from(primitives: Primitives) -> Self {
        primitives.0
    }
}

impl_into_enum!(i64 => AST:Int);
impl_into_enum!(f64 => AST:Float);
impl_into_enum!(bool => AST:Bool);
impl_into_enum!(String => AST:String);

impl Parse for Primitives {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let token = tokens.pop(pos)?;
        let primitive = match &token.value {
            TokenValue::Integer(int) => AST::Int(*int),
            TokenValue::Float(float) => AST::Float(*float),
            TokenValue::Boolean(boolean) => AST::Bool(*boolean),
            TokenValue::String(string) => AST::String(string.clone()),
            unexpected => {
                return Err(Pos {
                    start: token.start,
                    end: token.end,
                    value: Error::Unexpected {
                        unexpected,
                        expected: "integer",
                    }
                    .fatal(),
                })
            }
        };
        Ok(Pos {
            start: token.start,
            end: token.end,
            value: Primitives(primitive),
        })
    }
}
