use std::collections::HashMap;

use crate::checker::Type;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeType {
    #[default]
    Global,
    Function,
    Block,
    Loop,
    Struct,
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

    pub fn find_scope(&self, scope_type: ScopeType) -> Option<&Scope> {
        if let Some(parent) = self.parent.as_ref() {
            if parent.scope_type == scope_type {
                Some(parent)
            } else {
                parent.find_scope(scope_type)
            }
        } else {
            None
        }
    }

    pub fn parent(self) -> Box<Scope> {
        self.parent.unwrap()
    }

    pub fn add(&mut self, identifier: &str, value: Type) {
        self.values.insert(identifier.to_owned(), value);
    }

    pub fn add_without_shadow(&mut self, identifier: &str, value: Type) {
        self.values.entry(identifier.to_owned()).or_insert(value);
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
        self.lookup_local(identifier).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.lookup(identifier))
        })
    }

    pub fn lookup_local(&self, identifier: &String) -> Option<Type> {
        self.values.get(identifier).cloned()
    }
}
