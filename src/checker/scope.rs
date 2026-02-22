use std::{
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::checker::{Type, TypeResolver};

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeType {
    #[default]
    Global,
    Function,
    Closure,
    Block,
    MatchCase,
    Loop,
    Struct(usize),
}

#[derive(Default, Debug)]
pub struct Scope {
    pub types: Rc<TypeResolver>,
    scope_type: ScopeType,
    parent: Option<Box<Scope>>,
    values: HashMap<String, Type>,
    return_type: Option<Type>,
}

impl Scope {
    pub fn new(types: Rc<TypeResolver>) -> Self {
        Self {
            types,
            ..Default::default()
        }
    }

    pub fn derive(self: Box<Scope>, scope_type: ScopeType) -> Box<Scope> {
        let types = self.types.clone();
        Box::new(Self {
            scope_type,
            parent: Some(self),
            ..Self::new(types)
        })
    }

    pub fn derive_fn(self: Box<Scope>, return_type: &Type) -> Box<Scope> {
        let types = self.types.clone();
        Box::new(Self {
            scope_type: ScopeType::Function,
            parent: Some(self),
            return_type: Some(return_type.clone()),
            ..Self::new(types)
        })
    }

    pub fn return_type(&self) -> Option<&Type> {
        let function_scope =
            self.find_scope(|scope_type| matches!(scope_type, ScopeType::Function));
        function_scope.and_then(|scope| scope.return_type.as_ref())
    }

    pub fn within(&self, scope_type: ScopeType) -> bool {
        self.scope_type == scope_type
            || self
                .parent
                .as_ref()
                .map(|parent| parent.within(scope_type))
                .unwrap_or(false)
    }

    pub fn find_scope(&self, mut predicate: impl FnMut(ScopeType) -> bool) -> Option<&Scope> {
        if let Some(parent) = self.parent.as_ref() {
            if predicate(parent.scope_type) {
                Some(parent)
            } else {
                parent.find_scope(predicate)
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

    pub fn add_or(&mut self, identifier: &str, value: Type, if_present: impl Fn()) {
        let entry = self.values.entry(identifier.to_owned());
        if let Entry::Vacant(v) = entry {
            v.insert(value);
        } else {
            if_present();
        };
    }

    pub fn lookup(&self, identifier: &String) -> Option<Type> {
        self.lookup_local(identifier)
            .or_else(|| self.lookup_super(identifier))
    }

    pub fn lookup_local(&self, identifier: &String) -> Option<Type> {
        self.values.get(identifier).cloned()
    }

    fn lookup_super(&self, identifier: &String) -> Option<Type> {
        self.parent
            .as_ref()
            .and_then(|parent| parent.lookup(identifier))
    }
}
