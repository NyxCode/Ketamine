use crate::keywords::read_keyword;
use once_cell::unsync::Lazy;
use regex::Regex;

use std::fmt::{Display, Error, Formatter, Result as FmtResult};
use std::ops::Deref;
use std::str::FromStr;

mod keywords;
mod operator;

pub use operator::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub value: TokenValue,
}

#[derive(Debug)]
pub struct LexingError(usize);

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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),
    Operator(Operator),

    ParenthesesOpen,
    ParenthesesClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    Semicolon,
    Colon,
    Comma,
    Dot,

    Range,

    FunctionKeyword,
    ReturnKeyword,
    BreakKeyword,
    ContinueKeyword,
    IfKeyword,
    ElseKeyword,
    ForKeyword,
    InKeyword,
    LoopKeyword,
    WhileKeyword,
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        fn value(f: &mut Formatter<'_>, kind: &str, value: impl Display) -> FmtResult {
            write!(f, "{}<{}>", kind, value)
        }

        fn token(f: &mut Formatter<'_>, token: &str) -> FmtResult {
            write!(f, "{}", token)
        }

        match self {
            TokenValue::Integer(int) => value(f, "int", int),
            TokenValue::Float(float) => value(f, "float", float),
            TokenValue::String(string) => value(f, "string", string),
            TokenValue::Boolean(boolean) => value(f, "bool", boolean),
            TokenValue::Identifier(ident) => value(f, "identifier", ident),
            TokenValue::Operator(op) => value(f, "operator", op),
            TokenValue::ParenthesesOpen => token(f, "("),
            TokenValue::ParenthesesClose => token(f, ")"),
            TokenValue::BracketOpen => token(f, "["),
            TokenValue::BracketClose => token(f, "]"),
            TokenValue::BraceOpen => token(f, "{"),
            TokenValue::BraceClose => token(f, "}"),
            TokenValue::Semicolon => token(f, ";"),
            TokenValue::Comma => token(f, ","),
            TokenValue::Dot => token(f, "."),
            TokenValue::Colon => token(f, ":"),
            TokenValue::Range => token(f, ".."),
            TokenValue::FunctionKeyword => token(f, "function"),
            TokenValue::ReturnKeyword => token(f, "return"),
            TokenValue::BreakKeyword => token(f, "break"),
            TokenValue::ContinueKeyword => token(f, "continue"),
            TokenValue::ForKeyword => token(f, "for"),
            TokenValue::InKeyword => token(f, "in"),
            TokenValue::LoopKeyword => token(f, "loop"),
            TokenValue::WhileKeyword => token(f, "while"),
            TokenValue::IfKeyword => token(f, "if"),
            TokenValue::ElseKeyword => token(f, "else"),
        }
    }
}

impl Token {
    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

fn skip_whitespace(input: &str) -> usize {
    input
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(input.len())
}

fn read_integer(offset: usize, input: &str) -> Option<Token> {
    let regex = Lazy::new(|| Regex::new(r#"^\d+"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = i64::from_str(&input[m.start()..m.end()])
        .map(TokenValue::Integer)
        .expect("ICE");

    Some(Token {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_float(offset: usize, input: &str) -> Option<Token> {
    let regex = Lazy::new(|| Regex::new(r#"^\d+\.\d+"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = f64::from_str(m.as_str())
        .map(TokenValue::Float)
        .expect("ICE");

    Some(Token {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_ident(offset: usize, input: &str) -> Option<Token> {
    let regex = Lazy::new(|| Regex::new(r#"^[A-Za-z_]+[A-Za-z0-9_]*"#).unwrap());

    let m = regex.deref().find(input)?;
    let value = TokenValue::Identifier(m.as_str().to_owned());

    Some(Token {
        start: offset + m.start(),
        end: offset + m.end(),
        value,
    })
}

fn read_separator(offset: usize, input: &str) -> Option<Token> {
    let value = match input.chars().next()? {
        '(' => TokenValue::ParenthesesOpen,
        ')' => TokenValue::ParenthesesClose,
        '[' => TokenValue::BracketOpen,
        ']' => TokenValue::BracketClose,
        '{' => TokenValue::BraceOpen,
        '}' => TokenValue::BraceClose,
        _ => return None,
    };
    Some(Token {
        start: offset,
        end: offset + 1,
        value,
    })
}

fn read_range(offset: usize, input: &str) -> Option<Token> {
    if input.starts_with("..") {
        Some(Token {
            start: offset,
            end: offset + 2,
            value: TokenValue::Range,
        })
    } else {
        None
    }
}

fn read_semicolon(offset: usize, input: &str) -> Option<Token> {
    if input.chars().next()? == ';' {
        Some(Token {
            start: offset,
            end: offset + 1,
            value: TokenValue::Semicolon,
        })
    } else {
        None
    }
}

fn read_dot(offset: usize, input: &str) -> Option<Token> {
    if input.chars().next()? == '.' {
        Some(Token {
            start: offset,
            end: offset + 1,
            value: TokenValue::Dot,
        })
    } else {
        None
    }
}

fn read_comma(offset: usize, input: &str) -> Option<Token> {
    if input.chars().next()? == ',' {
        Some(Token {
            start: offset,
            end: offset + 1,
            value: TokenValue::Comma,
        })
    } else {
        None
    }
}

fn read_colon(offset: usize, input: &str) -> Option<Token> {
    if input.chars().next()? == ':' {
        Some(Token {
            start: offset,
            end: offset + 1,
            value: TokenValue::Colon,
        })
    } else {
        None
    }
}

fn read_string(offset: usize, input: &str) -> Option<Token> {
    // TODO: unescape pasrsed string ("\n" should not be parsed as "\\n")
    let regex = Lazy::new(|| Regex::new(r#"^"[^"\\]*(\\.[^"\\]*)*""#).unwrap());
    let m = regex.deref().find(input)?;
    let string = &input[(m.start() + 1)..(m.end() - 1)];
    let value = TokenValue::String(string.to_owned());

    Some(Token {
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
    type Item = Result<Token, LexingError>;

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
            .or_else(|| Operator::read_greedy(*pos, &input[*pos..]))
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexingError> {
    let mut iter = TokenIterator::new(input);
    let mut tokens = vec![];
    while let Some(token) = iter.next() {
        tokens.push(token?);
    }
    Ok(tokens)
}

#[test]
fn test() {
    let input = r#"for i in 0..3 {}"#;
    let tokens = tokenize(input).unwrap();
    println!("{:?}", tokens);
}
