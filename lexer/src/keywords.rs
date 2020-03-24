use crate::{read_semicolon, read_separator, Token, TokenValue};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::ops::Deref;

fn is_separate_token(input: &str) -> bool {
    input.is_empty()
        || input.chars().next().unwrap().is_whitespace()
        || read_separator(0, input).is_some()
        || read_semicolon(0, input).is_some()
}

pub(crate) fn read_keyword(offset: usize, input: &str) -> Option<Token> {
    static KEYWORDS: Lazy<HashMap<&'static str, TokenValue>> = Lazy::new(|| {
        let mut keywords = HashMap::new();
        keywords.insert("function", TokenValue::FunctionKeyword);
        keywords.insert("return", TokenValue::ReturnKeyword);
        keywords.insert("break", TokenValue::BreakKeyword);
        keywords.insert("continue", TokenValue::ContinueKeyword);
        keywords.insert("if", TokenValue::IfKeyword);
        keywords.insert("else", TokenValue::ElseKeyword);
        keywords.insert("for", TokenValue::ForKeyword);
        keywords.insert("in", TokenValue::InKeyword);
        keywords.insert("loop", TokenValue::LoopKeyword);
        keywords.insert("while", TokenValue::WhileKeyword);
        keywords.insert("true", TokenValue::Boolean(true));
        keywords.insert("false", TokenValue::Boolean(false));
        keywords
    });

    KEYWORDS
        .deref()
        .iter()
        .find(|(keyword, _value)| {
            let keyword: &'static str = keyword;
            input.starts_with(&keyword) && is_separate_token(&input[keyword.len()..])
        })
        .map(|(keyword, value)| Token {
            start: offset,
            end: offset + keyword.len(),
            value: value.clone(),
        })
}
