use crate::library::Library;
use crate::scope::ScopeStack;
use crate::values::Object;
use crate::values::{Array, ConcreteObject, Dictionary, NativeFunction, Value};
use crate::Evaluate;
use parser::ast::{Ident, Statement};
use parser::{Parse, Pos};
use std::collections::HashMap;
use std::ops::Deref;
use std::time::Instant;

mod assignment;
mod branching;
mod expression;
mod instructions;
mod literals;
mod loops;

#[test]
fn test() {
    let src = r#"
fibonacci = {
    cache: [],
    get_or_compute: function(n) {
        cached = this.cache[n];
        if (cached == null) {
            computed = if(n < 3) {
                1
            } else {
                this.get_or_compute(n - 2) + this.get_or_compute(n - 1)
            };
            this.cache[n] = computed;
            computed
        } else {
            cached
        }
    }
};

fibonacci.get_or_compute(60)
"#;
    let mut interpreter = Interpreter::new();

    crate::library::StandardLibrary.register(&mut interpreter);

    let print = NativeFunction::new(|this, args| {
        let content = args
            .into_iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", content);
        Ok(Value::Null)
    });
    interpreter
        .scope
        .push_var("print", Value::NativeFunction(print), false);

    let start = Instant::now();
    match interpreter.eval(src) {
        Ok(result) => println!("==> {}", result.to_string()),
        Err(err) => report::report(src, err.start, err.end, err.value),
    }
    println!(
        "    took {}s",
        Instant::now().duration_since(start).as_secs()
    );
}

pub struct Interpreter {
    pub(crate) scope: ScopeStack,
    pub(crate) integer_proto: Dictionary,
    pub(crate) float_proto: Dictionary,
    pub(crate) boolean_proto: Dictionary,
    pub(crate) string_proto: Dictionary,
    pub(crate) array_proto: Dictionary,
    pub(crate) object_proto: Dictionary,
    pub(crate) function_proto: Dictionary,
    pub(crate) null_proto: Dictionary,
}

impl Interpreter {
    fn new() -> Self {
        let mut scope = ScopeStack::readonly_root();

        let integer_proto = Dictionary::default();
        let float_proto = Dictionary::default();
        let boolean_proto = Dictionary::default();
        let string_proto = Dictionary::default();
        let array_proto = Dictionary::default();
        let object_proto = Dictionary::default();
        let function_proto = Dictionary::default();
        let null_proto = Dictionary::default();

        scope.push_var("$integer", Value::Dictionary(integer_proto.clone()), true);
        scope.push_var("$float", Value::Dictionary(float_proto.clone()), true);
        scope.push_var("$boolean", Value::Dictionary(boolean_proto.clone()), true);
        scope.push_var("$string", Value::Dictionary(string_proto.clone()), true);
        scope.push_var("$array", Value::Dictionary(array_proto.clone()), true);
        scope.push_var("$object", Value::Dictionary(object_proto.clone()), true);
        scope.push_var("$function", Value::Dictionary(function_proto.clone()), true);
        scope.push_var("$null", Value::Dictionary(null_proto.clone()), true);

        scope.push_scope(false);

        Interpreter {
            scope,
            integer_proto,
            float_proto,
            boolean_proto,
            string_proto,
            array_proto,
            object_proto,
            function_proto,
            null_proto,
        }
    }

    fn eval(&mut self, src: &str) -> Result<Value, Pos<String>> {
        let tokens = lexer::tokenize(src)
            .map_err(|err| Pos::new(err.0, err.0, format!("could not tokenize")))?;
        let mut tokens = &tokens[..];
        let statements = <Vec<Pos<Statement>>>::parse(0, &mut tokens)
            .map_err(|err| err.map(|x| x.into_inner().to_string()))?
            .value;

        Ok(statements.eval(self)?.into_value())
    }

    pub fn prototype_function<O, I, F>(&mut self, ident: I, function: F)
    where
        O: Object + ConcreteObject,
        I: Into<String>,
        F: Fn(O, Vec<Value>) -> Result<Value, String> + 'static,
    {
        let proto = O::get_prototype(self);
        let function = NativeFunction::new(move |this: Value, args: Vec<Value>| {
            function(O::try_get_as(this)?, args)
        });
        proto.insert(ident.into(), Value::NativeFunction(function));
    }

    pub fn prototype_field<O, I>(&mut self, ident: I, value: Value)
    where
        O: Object + ConcreteObject,
        I: Into<String>,
    {
        let proto = O::get_prototype(self);
        proto.insert(ident.into(), value);
    }

    pub fn get_proto(&self, val: &Value) -> &Dictionary {
        match val {
            Value::String(_) => &self.string_proto,
            Value::Integer(_) => &self.integer_proto,
            Value::Float(_) => &self.float_proto,
            Value::Boolean(_) => &self.boolean_proto,
            Value::Array(_) => &self.array_proto,
            Value::Dictionary(_) => &self.object_proto,
            Value::Function(_) => &self.function_proto,
            Value::NativeFunction(_) => &self.function_proto,
            Value::Null => &self.null_proto,
        }
    }

    pub fn get_prototype_field(&self, val: &Value, field: &str) -> Value {
        let prototype = self.get_proto(val);
        prototype.get_field(field).unwrap_or(Value::Null)
    }

    pub(crate) fn scope<T>(
        &mut self,
        readonly: bool,
        block: impl FnOnce(&mut Interpreter) -> T,
    ) -> T {
        self.scope.push_scope(readonly);
        let result = block(self);
        self.scope.pop_scope();
        result
    }
}
