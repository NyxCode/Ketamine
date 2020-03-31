use crate::error::{Error, Severity};
use crate::Pos;
use lexer::TokenValue;

pub trait TokenExt<'a> {
    fn peek(&self, pos: usize) -> Result<&'a Pos<TokenValue>, Pos<Severity<'a>>>;
    fn pop(&mut self, pos: usize) -> Result<&'a Pos<TokenValue>, Pos<Severity<'a>>>;
    fn pop_expect(
        &mut self,
        pos: usize,
        expect: &TokenValue,
    ) -> Result<&'a Pos<TokenValue>, Pos<Severity<'a>>> {
        let token = self.pop(pos)?;
        if &token.value != expect {
            Err(Pos {
                start: token.start,
                end: token.end,
                value: Error::Unexpected {
                    unexpected: &token.value,
                    expected: expect.name(),
                }
                .fatal(),
            })
        } else {
            Ok(token)
        }
    }
    fn pop_unwrap(&mut self) -> &'a Pos<TokenValue> {
        self.pop(0).unwrap()
    }
    fn peek_unwrap(&self) -> &'a Pos<TokenValue> {
        self.peek(0).unwrap()
    }
}

impl<'a> TokenExt<'a> for &'a [Pos<TokenValue>] {
    fn peek(&self, pos: usize) -> Result<&'a Pos<TokenValue>, Pos<Severity<'a>>> {
        if self.is_empty() {
            Err(Pos {
                start: pos,
                end: pos + 1,
                value: Error::Missing("token").fatal(),
            })
        } else {
            let token = &self[0];
            Ok(token)
        }
    }

    fn pop(&mut self, pos: usize) -> Result<&'a Pos<TokenValue>, Pos<Severity<'a>>> {
        let token = self.peek(pos)?;
        *self = &self[1..];
        Ok(token)
    }
}
