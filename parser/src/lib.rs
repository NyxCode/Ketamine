#![feature(associated_type_defaults)]
#![feature(type_name_of_val)]

pub mod ast;
pub mod error;
pub mod macros;
pub mod token_ext;
pub mod tree;

use crate::ast::AST;
use crate::error::{Error, ParseResult, ResultExt, Severity};
use crate::token_ext::TokenExt;

pub use lexer::Pos;
use lexer::TokenValue;

/*
fib = function(n) {
            if (n < 3) {
                1
            } else {
                fib(n - 2) + fib(n - 1)
            }
        };
*/

#[test]
fn test() {
    use crate::ast::Statement;
    use crate::tree::TreeDisplay;
    use lexer::tokenize;
    use ptree::TreeBuilder;

    let input = r#"
        if (true) {

        };
        print("lol");
    "#;
    let mut tokens = &tokenize(input).unwrap()[..];
    match <Vec<Pos<Statement>>>::parse(0, &mut tokens) {
        Ok(statements) => {
            let mut tree = TreeBuilder::new("code".to_owned());
            for statement in statements.value {
                statement.value.display(&mut tree);
            }
            ptree::print_tree(&tree.build()).unwrap();
        }
        Err(err) => {
            report::report(input, err.start, err.end, err.value.into_inner());
        }
    }
}

fn find_closing_delimiter<'a>(
    mut pos: usize,
    mut tokens: &'a [Token],
    open: &TokenValue,
    close: &TokenValue,
    mut current_count: usize,
) -> Result<usize, Pos<Severity<'a>>> {
    let mut index = 0;
    loop {
        let next = &tokens
            .pop(pos)
            .map_err(|p| p.map(|_| Error::Missing(close.name()).fatal()))?;
        match &next.value {
            x if x == open => current_count += 1,
            x if x == close => current_count -= 1,
            _ => (),
        }
        pos = next.end;
        if current_count == 0 {
            return Ok(index);
        }
        index += 1;
    }
}

fn parse_list<'a, V>(
    mut pos: usize,
    tokens: &mut &'a [Token],
    open: TokenValue,
    close: TokenValue,
    delimiter: TokenValue,
) -> ParseResult<'a, Vec<Pos<V>>>
where
    V: Parse,
{
    let open_token = tokens.pop_expect(pos, &open).into_recoverable()?;
    pos = open_token.end;
    let mut list = vec![];
    let close_token = loop {
        let next = tokens.peek(pos)?;
        if next.value == close {
            tokens.pop_unwrap();
            break next;
        }

        let element = V::parse(pos, tokens)?;
        pos = element.end;
        list.push(element);

        let next = tokens.peek(pos)?;
        if next.value == close {
            tokens.pop_unwrap();
            break next;
        } else if next.value == delimiter {
            tokens.pop_unwrap();
        }
    };

    Ok(Pos {
        start: open_token.start,
        end: close_token.end,
        value: list,
    })
}

pub type Token = Pos<TokenValue>;

pub trait Parse: Sized {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self>;
}

trait TryParse: Sized {
    fn try_parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self>;
}

impl<T> TryParse for T
where
    T: Parse,
{
    fn try_parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut try_tokens = *tokens;
        let parsed = Self::parse(pos, &mut try_tokens)?;
        *tokens = try_tokens;
        Ok(parsed)
    }
}
