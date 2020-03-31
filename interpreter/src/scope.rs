use crate::values::Value;

use std::collections::HashMap;

#[derive(Debug)]
pub struct ScopeStack {
    scopes: Vec<Scope>,
}

#[derive(Debug)]
struct Scope {
    variables: HashMap<String, Value>,
    readonly: bool,
}

impl Scope {
    pub fn new(readonly: bool) -> Self {
        Scope {
            variables: HashMap::new(),
            readonly,
        }
    }
}

impl ScopeStack {
    pub fn readonly_root() -> ScopeStack {
        let root = Scope::new(true);
        ScopeStack { scopes: vec![root] }
    }

    pub fn push_var(&mut self, ident: impl Into<String>, var: Value, force: bool) -> Option<Value> {
        let scope = self
            .scopes
            .iter_mut()
            .rev()
            .find(|scope| force || !scope.readonly)
            .unwrap();
        scope.variables.insert(ident.into(), var)
    }

    pub fn get_var(&self, ident: &str) -> Option<&Value> {
        self.scopes
            .iter()
            .rev()
            .flat_map(|scope| scope.variables.get(ident))
            .next()
    }

    pub fn push_scope(&mut self, readonly: bool) {
        let scope = Scope {
            variables: HashMap::new(),
            readonly,
        };
        self.scopes.push(scope);
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop().unwrap();
    }
}
