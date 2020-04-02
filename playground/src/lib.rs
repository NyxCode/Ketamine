use std::ops::Deref;
use std::rc::Rc;

use serde::Serialize;
use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::prelude::*;

use interpreter::{Interpreter, NativeFunction, Object, Value};
use interpreter::library::Library;
use lexer::{LexingError, Pos, TokenValue};
use parser::Parse;
use parser::ast::Statement;
use parser::error::Severity;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(src: &str) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let result = run_interpreter(&src).map(|(val, stdout)| (val.to_string(), stdout))?;
    Ok(JsValue::from_serde(&result).unwrap())
}

#[wasm_bindgen]
pub fn lex(src: &str) -> Result<JsValue, JsValue> {
    let tokens = run_lexer(src)?;
    Ok(JsValue::from_serde(&tokens).unwrap())
}

#[wasm_bindgen]
pub fn parse(src: &str) -> Result<JsValue, JsValue> {
    let tokens = run_lexer(src)?;
    let mut tokens = &tokens[..];
    let ast = run_parser(src, &mut tokens)?;
    Ok(JsValue::from_serde(&ast).unwrap())
}

fn run_interpreter(src: &str) -> Result<(Value, String), JsValue> {
    let mut inter = Interpreter::new();
    let lib = BrowserLib::default();
    lib.register(&mut inter);
    interpreter::library::StandardLibrary.register(&mut inter);
    inter.eval(src)
        .map_err(|Pos { start, end, value }| {
            let report = report::report_string(src, start, end, value);
            PlaygroundError { start, end, report }
        })
        .map_err(|err| JsValue::from_serde(&err).unwrap())
        .map(|val| (val, lib.clone_stdout()))
}

fn run_parser(src: &str, tokens: &mut &[Pos<TokenValue>]) -> Result<Vec<Pos<Statement>>, JsValue> {
    <Vec<Pos<Statement>>>::parse(0, tokens)
        .map(|code| code.value)
        .map_err(|err| err.map(Severity::into_inner))
        .map_err(|Pos { start, end, value }| {
            let report = report::report_string(src, start, end, value);
            PlaygroundError { start, end, report }
        })
        .map_err(|err| JsValue::from_serde(&err).unwrap())
}

fn run_lexer(src: &str) -> Result<Vec<Pos<TokenValue>>, JsValue> {
    lexer::tokenize(src)
        .map_err(|LexingError(pos)| {
            let report = report::report_string(src, pos, pos + 1, "illegal token");
            PlaygroundError { start: pos, end: pos + 1, report }
        })
        .map_err(|err| JsValue::from_serde(&err).unwrap())
}

#[derive(Serialize)]
struct PlaygroundError {
    start: usize,
    end: usize,
    report: String,
}

struct BrowserLib {
    out: Rc<RefCell<String>>
}

impl BrowserLib {
    fn clone_stdout(&self) -> String {
        self.out.deref().borrow().clone()
    }
}

impl Default for BrowserLib {
    fn default() -> Self {
        BrowserLib {
            out: Rc::new(RefCell::new(String::new()))
        }
    }
}

impl Library for BrowserLib {
    fn register(&self, inter: &mut Interpreter) {
        let out = self.out.clone();
        let print = NativeFunction::new(move |_, args| {
            let msg = args.into_iter().map(|arg| arg.to_string()).collect::<Vec<_>>().join(" ");
            let mut out = out.deref().borrow_mut();
            out.push_str(&msg);
            out.push_str("\n");
            Ok(Value::Null)
        });
        inter.scope.push_var("print", Value::NativeFunction(print), false);
    }
}