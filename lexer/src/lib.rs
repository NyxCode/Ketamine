use crate::keywords::read_keyword;
use once_cell::unsync::Lazy;
use regex::Regex;

use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::str::FromStr;

mod keywords;
mod operator;
mod token;

pub use operator::*;
pub use token::*;

#[derive(Debug)]
pub struct LexingError(pub usize);

impl Display for LexingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "failed to parse token")
    }
}

impl LexingError {
    pub fn location(&self) -> usize {
        self.0
    }
}

fn skip_whitespace(input: &str) -> usize {
    input
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(input.len())
}

fn read_integer(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    let regex = Lazy::new(|| Regex::new(r#"^\d+"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = i64::from_str(&input[m.start()..m.end()])
        .map(TokenValue::Integer)
        .expect("ICE");

    Some(Pos {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_float(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    let regex = Lazy::new(|| Regex::new(r#"^\d+\.\d+"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = f64::from_str(m.as_str())
        .map(TokenValue::Float)
        .expect("ICE");

    Some(Pos {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_ident(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    let regex = Lazy::new(|| Regex::new(r#"^[$A-Za-z_]+[$A-Za-z0-9_]*"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = TokenValue::Identifier(m.as_str().to_owned());

    Some(Pos {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_separator(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    let value = match input.chars().next()? {
        '(' => TokenValue::ParenthesesOpen,
        ')' => TokenValue::ParenthesesClose,
        '[' => TokenValue::BracketOpen,
        ']' => TokenValue::BracketClose,
        '{' => TokenValue::BraceOpen,
        '}' => TokenValue::BraceClose,
        _ => return None,
    };
    Some(Pos {
        start: offset,
        end: offset + 1,
        value,
    })
}

fn read_range(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    if input.starts_with("..") {
        Some(Pos {
            start: offset,
            end: offset + 2,
            value: TokenValue::Range,
        })
    } else {
        None
    }
}

fn read_semicolon(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    if input.chars().next()? == ';' {
        Some(Pos {
            start: offset,
            end: offset + 1,
            value: TokenValue::Semicolon,
        })
    } else {
        None
    }
}

fn read_dot(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    if input.chars().next()? == '.' {
        Some(Pos {
            start: offset,
            end: offset + 1,
            value: TokenValue::Dot,
        })
    } else {
        None
    }
}

fn read_comma(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    if input.chars().next()? == ',' {
        Some(Pos {
            start: offset,
            end: offset + 1,
            value: TokenValue::Comma,
        })
    } else {
        None
    }
}

fn read_colon(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    if input.chars().next()? == ':' {
        Some(Pos {
            start: offset,
            end: offset + 1,
            value: TokenValue::Colon,
        })
    } else {
        None
    }
}

fn read_string(offset: usize, input: &str) -> Option<Pos<TokenValue>> {
    // TODO: unescape pasrsed string ("\n" should not be parsed as "\\n")
    let regex = Lazy::new(|| Regex::new(r#"^"[^"\\]*(\\.[^"\\]*)*""#).unwrap());
    let m = regex.deref().find(input)?;
    let string = &input[(m.start() + 1)..(m.end() - 1)];
    let value = TokenValue::String(string.to_owned());

    Some(Pos {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

pub struct TokenIterator<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        TokenIterator { input, pos: 0 }
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<Pos<TokenValue>, LexingError>;

    fn next(&mut self) -> Option<Self::Item> {
        let input: &str = self.input;
        let pos: &mut usize = &mut self.pos;

        *pos += skip_whitespace(&input[*pos..]);
        if *pos >= input.len() {
            return None;
        }

        let token = None
            .or_else(|| read_semicolon(*pos, &input[*pos..]))
            .or_else(|| read_comma(*pos, &input[*pos..]))
            .or_else(|| read_range(*pos, &input[*pos..]))
            .or_else(|| read_dot(*pos, &input[*pos..]))
            .or_else(|| read_colon(*pos, &input[*pos..]))
            .or_else(|| read_separator(*pos, &input[*pos..]))
            .or_else(|| read_keyword(*pos, &input[*pos..]))
            .or_else(|| read_operator(*pos, &input[*pos..]))
            .or_else(|| read_string(*pos, &input[*pos..]))
            .or_else(|| read_float(*pos, &input[*pos..]))
            .or_else(|| read_integer(*pos, &input[*pos..]))
            .or_else(|| read_ident(*pos, &input[*pos..]));

        if let Some(token) = token {
            *pos = token.end;
            Some(Ok(token))
        } else {
            Some(Err(LexingError(*pos)))
        }
    }
}

pub fn tokenize(input: &str) -> Result<Vec<Pos<TokenValue>>, LexingError> {
    let mut tokens = vec![];
    for token in TokenIterator::new(input) {
        tokens.push(token?);
    }
    Ok(tokens)
}
