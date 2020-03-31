#![deny(unused_mut, unreachable_patterns)]

use crate::scope::ScopeStack;
use crate::values::{Array, ConcreteObject, Dictionary, NativeFunction, Object, Value};
use lexer::{tokenize, Pos};
use parser::ast::{
    Assignment, BinaryOperator, Break, Call, Continue, FieldAccess, ForLoop, Function, Ident,
    IfBranch, Index, Parentheses, Range, Return, Statement, UnaryOperator, AST,
};
use parser::Parse;
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::ops::Deref;

mod interpreter;
mod library;
mod scope;
mod values;

pub use crate::interpreter::*;
use std::rc::Rc;

trait Evaluate {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>>;
}

impl Evaluate for Pos<Box<AST>> {
    fn eval(self, scope: &mut Interpreter) -> Result<Eval, Pos<String>> {
        self.map(|b| *b).eval(scope)
    }
}

impl Evaluate for Pos<AST> {
    fn eval(self, interp: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let (a, b) = (self.start, self.end);

        match self.value {
            AST::Int(int) => Ok(Eval::Value(Value::Integer(int))),
            AST::Float(float) => Ok(Eval::Value(Value::Float(float))),
            AST::Bool(boolean) => Ok(Eval::Value(Value::Boolean(boolean))),
            AST::String(string) => Ok(Eval::Value(Value::String(string))),
            AST::Assignment(assign) => Pos::new(a, b, assign).eval(interp),
            AST::UnaryOperation(unary) => Pos::new(a, b, unary).eval(interp),
            AST::BinaryOperation(op) => Pos::new(a, b, op).eval(interp),
            AST::ForLoop(for_loop) => Pos::new(a, b, for_loop).eval(interp),
            AST::WhileLoop(while_loop) => Pos::new(a, b, while_loop).eval(interp),
            AST::If(if_expr) => Pos::new(a, b, if_expr).eval(interp),
            AST::Object(object) => Pos::new(a, b, object).eval(interp),
            AST::List(list) => Pos::new(a, b, list).eval(interp),
            AST::Range(range) => Pos::new(a, b, range).eval(interp),
            AST::Return(instruction) => Pos::new(a, b, instruction).eval(interp),
            AST::Break(instruction) => Pos::new(a, b, instruction).eval(interp),
            AST::Continue(..) => Ok(Eval::Continue),
            AST::Parentheses(Parentheses(inner)) => inner.eval(interp),
            AST::Call(call) => Pos::new(a, b, call).eval(interp),
            AST::Index(index) => Pos::new(a, b, index).eval(interp),
            AST::FieldAccess(access) => Pos::new(a, b, access).eval(interp),
            AST::Ident(Ident(ident)) => {
                let val = interp.scope.get_var(&ident).cloned().unwrap_or(Value::Null);
                Ok(Eval::Value(val))
            }
            AST::Function(function) => {
                let function = values::Function {
                    function: Rc::new(function),
                };
                Ok(Eval::Value(Value::Function(function)))
            }
        }
    }
}

impl Evaluate for Pos<Call> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Call { value, args },
        } = self;
        let (this, function) = match value.map(|x| *x) {
            // value.field()
            // ^this^function^
            Pos {
                start,
                end,
                value: AST::FieldAccess(FieldAccess { value, field }),
            } => {
                let this = value
                    .eval(interpreter)?
                    .try_into_value()
                    .map_err(|err| Pos::new(start, end, err))?;
                let function = this
                    .get_field(&field.value)
                    .or_else(|| interpreter.get_proto(&this).get_field(&field.value))
                    .unwrap_or(Value::Null);
                (this, function)
            }
            // (x)()
            other => {
                let function = other
                    .eval(interpreter)?
                    .try_into_value()
                    .map_err(|err| Pos::new(start, end, err))?;
                (Value::Null, function)
            }
        };

        let mut arg_values = Vec::with_capacity(args.len());
        for arg in args {
            let arg = arg.eval(interpreter)?.into_value();
            arg_values.push(arg);
        }
        let result = function.call(start, end, interpreter, this, arg_values)?;
        Ok(Eval::Value(result))
    }
}

impl Evaluate for Pos<FieldAccess> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: FieldAccess { value, field },
        } = self;
        let value = value
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;
        let field = value
            .get_field(&field.value)
            .or_else(|| interpreter.get_proto(&value).get_field(&field.value))
            .unwrap_or(Value::Null);
        Ok(Eval::Value(field))
    }
}

impl Evaluate for Pos<Index> {
    fn eval(self, interpreter: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let Pos {
            start,
            end,
            value: Index { value, index },
        } = self;
        let value = value
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;
        let idx = index
            .eval(interpreter)?
            .try_into_value()
            .map_err(|err| Pos::new(start, end, err))?;
        let result = value.get_index(&idx).ok_or_else(|| {
            let msg = format!(
                "can't index into {} using {}",
                value.type_name(),
                idx.type_name()
            );
            Pos::new(start, end, msg)
        })?;
        Ok(Eval::Value(result))
    }
}

impl Evaluate for Pos<Statement> {
    fn eval(self, scope: &mut Interpreter) -> Result<Eval, Pos<String>> {
        self.map(|statement| *statement.into_inner()).eval(scope)
    }
}

impl Evaluate for Vec<Pos<Statement>> {
    fn eval(self, scope: &mut Interpreter) -> Result<Eval, Pos<String>> {
        let len = self.len();
        for (idx, statement) in self.into_iter().enumerate() {
            let is_unterminated = if let Statement::Unterminated(..) = &statement.value {
                true
            } else {
                false
            };
            let result = match statement.eval(scope)? {
                val @ Eval::Value(..) => val,
                instruction => return Ok(instruction),
            };
            if idx == len - 1 && is_unterminated {
                return Ok(result);
            }
        }

        Ok(Eval::Value(Value::Null))
    }
}

#[derive(Debug)]
enum Eval {
    Return(Value),
    Break(Value),
    Continue,
    Value(Value),
}

impl Eval {
    fn try_into_value(self) -> Result<Value, String> {
        match self {
            Eval::Value(val) => Ok(val),
            other => Err(format!("expected a value, got {:?}", other)),
        }
    }

    fn into_value(self) -> Value {
        match self {
            Eval::Return(val) => val,
            Eval::Break(val) => val,
            Eval::Continue => Value::Null,
            Eval::Value(val) => val,
        }
    }
}
