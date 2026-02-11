use std::collections::HashMap;

use crate::checker::Type;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeType {
    Global,
    Function,
    Block,
    Loop,
    Struct,
}

impl Default for ScopeType {
    fn default() -> Self {
        ScopeType::Global
    }
}

#[derive(Default, Debug)]
pub struct Scope {
    scope_type: ScopeType,
    parent: Option<Box<Scope>>,
    values: HashMap<String, Type>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn derive(self: Box<Scope>, scope_type: ScopeType) -> Box<Scope> {
        Box::new(Self {
            scope_type,
            parent: Some(self),
            ..Self::new()
        })
    }

    pub fn within(&self, scope_type: ScopeType) -> bool {
        self.scope_type == scope_type
            || self
                .parent
                .as_ref()
                .map(|parent| parent.within(scope_type))
                .unwrap_or(false)
    }

    pub fn parent(self) -> Box<Scope> {
        self.parent.unwrap()
    }

    pub fn add(&mut self, identifier: &String, value: Type) {
        self.values.insert(identifier.clone(), value);
    }

    pub fn add_without_shadow(&mut self, identifier: &String, value: Type) {
        self.values.entry(identifier.clone()).or_insert(value);
    }

    pub fn contains(&self, identifier: &String) -> bool {
        self.values.contains_key(identifier)
            || self
                .parent
                .as_ref()
                .map(|parent| parent.contains(identifier))
                .unwrap_or(false)
    }

    pub fn lookup(&self, identifier: &String) -> Option<Type> {
        self.values.get(identifier).cloned().or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.lookup(identifier))
        })
    }
}
