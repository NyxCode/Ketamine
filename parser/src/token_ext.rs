use lexer::TokenValue;
use crate::Parsed;
use crate::error::{Error, Severity};

pub trait TokenExt<'a> {
    fn peek(&self, pos: usize) -> Result<&'a Parsed<TokenValue>, Parsed<Severity<'a>>>;
    fn pop(&mut self, pos: usize) -> Result<&'a Parsed<TokenValue>, Parsed<Severity<'a>>>;
    fn pop_expect(&mut self, pos: usize, expect: &TokenValue) -> Result<&'a Parsed<TokenValue>, Parsed<Severity<'a>>> {
        let token = self.pop(pos)?;
        if &token.value != expect {
            Err(Parsed {
                start: token.start,
                end: token.end,
                value: Error::Unexpected { unexpected: &token.value, expected: expect.name() }.fatal()
            })
        } else {
            Ok(token)
        }
    }
    fn pop_unwrap(&mut self) -> &'a Parsed<TokenValue> {
        self.pop(0).unwrap()
    }
    fn peek_unwrap(&self) -> &'a Parsed<TokenValue> {
        self.peek(0).unwrap()
    }
}

impl <'a> TokenExt<'a> for &'a [Parsed<TokenValue>] {
    fn peek(&self, pos: usize) -> Result<&'a Parsed<TokenValue>, Parsed<Severity<'a>>> {
        if self.is_empty() {
            Err(Parsed {
                start: pos,
                end: pos + 1,
                value: Error::Missing("token").fatal()
            })
        } else {
            let token = &self[0];
            Ok(token)
        }
    }

    fn pop(&mut self, pos: usize) -> Result<&'a Parsed<TokenValue>, Parsed<Severity<'a>>> {
        let token = self.peek(pos)?;
        *self = &self[1..];
        Ok(token)
    }
}