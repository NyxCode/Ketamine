#![feature(associated_type_defaults)]
#![feature(type_name_of_val)]

mod error;
mod token_ext;
mod macros;
mod tree;

pub use lexer::Parsed;
use crate::error::{Error, ParseResult, Severity, ResultExt};
use crate::token_ext::TokenExt;
use lexer::{TokenValue, tokenize};
use crate::tree::TreeDisplay;
use ptree::TreeBuilder;

first_value_of!(
    AtomicValues:
    Return,
    Function,
    If,
    Ident,
    Primitives,
);

#[test]
fn test() {
    let input = r#"
fib = function(n) {
    if (n) {
        1
    }
};
    "#;
    let mut tokens = &tokenize(input).unwrap()[..];
    match <Vec<Parsed<Statement>>>::parse(0, &mut tokens) {
        Ok(statements) => {
            let mut tree = TreeBuilder::new("code".to_owned());
            for statement in statements.value {
                statement.value.display(&mut tree);
            }
            ptree::print_tree(&tree.build()).unwrap();
        },
        Err(err) => {
            report::report(input, err.start, err.end, err.value.into_inner());
        }
    }
}


#[derive(Debug)]
struct Primitives(AST);

impl From<Primitives> for AST {
    fn from(primitives: Primitives) -> Self {
        primitives.0
    }
}

#[derive(Debug)]
enum BinaryOperator {
    Add,
    Mul,
}

#[derive(Debug)]
struct BinaryOperation {
    lhs: Box<AST>,
    op: BinaryOperator,
    rhs: Box<AST>,
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
            unexpected => return Err(Parsed {
                start: token.start,
                end: token.end,
                value: Error::Unexpected { unexpected, expected: "integer" }.fatal(),
            })
        };
        Ok(Parsed {
            start: token.start,
            end: token.end,
            value: Primitives(primitive),
        })
    }
}

#[derive(Debug)]
struct Ident(String);
impl_into_enum!(Ident => AST:Ident);

impl Parse for Ident {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let token = tokens.pop(pos)?;
        match token.value {
            TokenValue::Identifier(ref ident) => Ok(Parsed {
                start: token.start,
                end: token.end,
                value: Ident(ident.clone()),
            }),
            ref unexpected => Err(Parsed {
                start: token.start,
                end: token.end,
                value: Error::Unexpected { unexpected, expected: "identifier" }.recoverable(),
            })
        }
    }
}

#[derive(Debug)]
struct Assignment {
    receiver: Parsed<Box<AST>>,
    value: Parsed<Box<AST>>,
}
impl_into_enum!(Assignment => AST:Assignment);

#[derive(Debug)]
struct Return(Option<Parsed<Box<AST>>>);
impl_into_enum!(Return => AST:Return);

#[derive(Debug)]
struct Break(Option<Parsed<AST>>);

#[derive(Debug)]
struct Continue();

#[derive(Debug)]
struct FieldAccess {
    value: Parsed<Box<AST>>,
    field: Parsed<Ident>,
}
impl_into_enum!(FieldAccess => AST:FieldAccess);

#[derive(Debug)]
struct Function {
    args: Vec<Parsed<Ident>>,
    body: Vec<Parsed<Statement>>,
}
impl_into_enum!(Function => AST:Function);

#[derive(Debug)]
struct Index {
    value: Parsed<Box<AST>>,
    index: Parsed<Box<AST>>,
}
impl_into_enum!(Index => AST:Index);

#[derive(Debug)]
struct If {
    if_branch: IfBranch,
    else_if_branches: Vec<IfBranch>,
    else_branch: Option<Vec<Parsed<Statement>>>
}
impl_into_enum!(If => AST:If);


#[derive(Debug)]
struct IfBranch {
    condition: Parsed<Box<AST>>,
    body: Vec<Parsed<Statement>>
}

#[derive(Debug)]
enum AST {
    Ident(Ident),
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Return(Return),
    Assignment(Assignment),
    FieldAccess(FieldAccess),
    Index(Index),
    Function(Function),
    If(If)
}

#[derive(Debug)]
enum Statement {
    Unterminated(Box<AST>),
    Terminated(Box<AST>),
}

impl Parse for Statement {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut statement: Parsed<AST> = AtomicValues::parse(pos, tokens)?.map(AST::from);

        loop {
            match tokens.peek(pos).ok() {
                Some(Parsed { value: TokenValue::Semicolon, end, .. }) => {
                    tokens.pop_unwrap();
                    return Ok(statement.map(Box::new).map(Statement::Terminated));
                }
                None => {
                    return Ok(statement.map(Box::new).map(Statement::Unterminated));
                }
                Some(next) => {
                    statement = AST::append(statement, tokens)?.map(Into::<AST>::into)
                }
            };
        }

        unreachable!()
    }
}
impl Parse for Vec<Parsed<Statement>> {
    fn parse<'a>(mut pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let start = pos;
        let mut statements = vec![];
        while !tokens.is_empty() {
            let statement = Statement::parse(pos, tokens)?;
            pos = statement.end;
            statements.push(statement);
        }
        Ok(Parsed {
            start,
            end: pos,
            value: statements
        })
    }
}

impl Parse for IfBranch {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens.pop_expect(pos, &TokenValue::IfKeyword).into_recoverable()?;
        let par_open = tokens.pop_expect(keyword.end, &TokenValue::ParenthesesOpen)?;
        let par_close_idx = find_closing_delimiter(par_open.end, tokens, &TokenValue::ParenthesesOpen, &TokenValue::ParenthesesClose, 1).into_fatal()?;
        let mut condition_tokens = &tokens[..par_close_idx];
        let condition = AST::parse(par_open.end, &mut condition_tokens).into_fatal()?;
        assert!(condition_tokens.is_empty());
        *tokens = &tokens[par_close_idx..];
        let par_close = tokens.pop_expect(condition.end, &TokenValue::ParenthesesClose)?;
        let brace_open = tokens.pop_expect(par_close.end, &TokenValue::BraceOpen)?;
        let brace_close_idx = find_closing_delimiter(brace_open.end, tokens, &TokenValue::BraceOpen, &TokenValue::BraceClose, 1).into_fatal()?;
        let mut body_tokens = &tokens[..brace_close_idx];
        let body = <Vec<Parsed<Statement>>>::parse(brace_open.end, &mut body_tokens)?;
        assert!(body_tokens.is_empty());
        *tokens = &tokens[brace_close_idx..];
        let brace_close = tokens.pop_expect(body.end, &TokenValue::BraceClose)?;

        Ok(Parsed {
            start: keyword.start,
            end: brace_close.end,
            value: IfBranch {
                condition: condition.map(Box::new),
                body: body.value
            }
        })
    }
}

impl Parse for If {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let if_branch = IfBranch::parse(pos, tokens)?;

        Ok(Parsed {
            start: if_branch.start,
            end: if_branch.end,
            value: If {
                if_branch: if_branch.value,
                else_if_branches: vec![],
                else_branch: None
            }
        })
    }
}

impl Parse for Return {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens.pop_expect(pos, &TokenValue::ReturnKeyword).into_recoverable()?;
        let next = match tokens.peek(keyword.end).ok() {
            Some(next) => next,
            None | Some(Parsed { value: TokenValue::Semicolon, .. }) => return Ok(keyword.clone().map(|_| Return(None))),
        };

        let value = AST::parse(next.start, tokens)?;
        Ok(Parsed::new(keyword.start, value.end, Return(Some(value.map(Box::new)))))
    }
}

impl Parse for Function {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let keyword = tokens.pop_expect(pos, &TokenValue::FunctionKeyword)
            .into_recoverable()?;
        let args = parse_list(
            keyword.end,
            tokens,
            TokenValue::ParenthesesOpen,
            TokenValue::ParenthesesClose,
            TokenValue::Comma,
        ).into_fatal()?;
        let brace_open = tokens.pop_expect(args.end, &TokenValue::BraceOpen)?;
        let brace_close_idx = find_closing_delimiter(
            brace_open.end,
            tokens,
            &TokenValue::BraceOpen,
            &TokenValue::BraceClose,
            1
        )?;
        let mut body_tokens = &tokens[..brace_close_idx];
        let mut body = vec![];
        let mut body_end = brace_open.end;
        while !body_tokens.is_empty() {
            let statement = Statement::parse(pos, &mut body_tokens).into_fatal()?;
            body_end = statement.end;
            body.push(statement);
        }

        *tokens = &tokens[brace_close_idx..];
        let brace_close = tokens.pop_expect(body_end, &TokenValue::BraceClose)?;

        Ok(Parsed {
            start: keyword.start,
            end: brace_close.end,
            value: Function {
                args: args.value,
                body
            }
        })
    }
}

impl Parse for AST {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut ast: Parsed<AST> = AtomicValues::parse(pos, tokens)?.map(AST::from);

        loop {
            match tokens.peek(pos).ok() {
                None |
                Some(Parsed { value: TokenValue::Semicolon, .. }) |
                Some(Parsed { value: TokenValue::ParenthesesClose, .. }) |
                Some(Parsed { value: TokenValue::BraceClose, .. }) |
                Some(Parsed { value: TokenValue::BracketClose, .. }) => return Ok(ast),
                Some(next) => ast = AST::append(ast, tokens)?,
            };
        }

        unreachable!()
    }
}

impl AST {
    fn append<'a>(prev: Parsed<Self>, tokens: &mut &'a [Token]) -> ParseResult<'a, AST> {
        let next = tokens.peek_unwrap();
        match &next.value {
            TokenValue::Assign => {
                tokens.pop_unwrap();
                let next = AST::parse(prev.end, tokens)?;
                Ok(Parsed {
                    start: prev.start,
                    end: next.end,
                    value: AST::Assignment(Assignment {
                        receiver: prev.map(Box::new),
                        value: next.map(Box::new),
                    }),
                })
            }
            TokenValue::Dot => {
                tokens.pop_unwrap();
                let field = Ident::parse(prev.end, tokens)?;
                Ok(Parsed {
                    start: prev.start,
                    end: field.end,
                    value: AST::FieldAccess(FieldAccess {
                        value: prev.map(Box::new),
                        field,
                    }),
                })
            }
            TokenValue::BracketOpen => {
                let open = tokens.pop_unwrap();
                let index = AST::parse(open.end, tokens)?.map(Box::new);
                let close = tokens.pop_expect(index.end, &TokenValue::BracketClose)?;
                Ok(Parsed {
                    start: open.start,
                    end: close.end,
                    value: AST::Index(Index {
                        value: prev.map(Box::new),
                        index,
                    }),
                })
            }
            other => unreachable!("{:?}", other)
        }
    }
}

fn find_closing_delimiter<'a>(
    mut pos: usize,
    mut tokens: &'a [Token],
    open: &TokenValue,
    close: &TokenValue,
    mut current_count: usize,
) -> Result<usize, Parsed<Severity<'a>>> {
    let mut index = 0;
    loop {
        let next = &tokens.pop(pos)
            .map_err(|p| p.map(|_| Error::Missing(close.name()).fatal()))?;
        match &next.value {
            x if x == open => current_count += 1,
            x if x == close => current_count -= 1,
            _ => ()
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
) -> ParseResult<'a, Vec<Parsed<V>>>
    where V: Parse {
    let open_token = tokens.pop_expect(pos, &open)?;
    pos = open_token.end;
    let mut list = vec![];
    let close_token = loop {
        let next = tokens.peek(pos)?;
        if &next.value == &close {
            tokens.pop_unwrap();
            break next;
        }

        let element = V::parse(pos, tokens)?;
        pos = element.end;
        list.push(element);

        let next = tokens.peek(pos)?;
        if &next.value == &close {
            tokens.pop_unwrap();
            break next;
        } else if &next.value == &delimiter {
            tokens.pop_unwrap();
        }
    };

    Ok(Parsed {
        start: open_token.start,
        end: close_token.end,
        value: list,
    })
}

pub type Token = Parsed<TokenValue>;

trait Parse: Sized {
    fn parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self>;
}

trait TryParse: Sized {
    fn try_parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self>;
}

impl<T> TryParse for T where T: Parse {
    fn try_parse<'a>(pos: usize, tokens: &mut &'a [Token]) -> ParseResult<'a, Self> {
        let mut try_tokens = *tokens;
        let parsed = Self::parse(pos, &mut try_tokens)?;
        *tokens = try_tokens;
        Ok(parsed)
    }
}