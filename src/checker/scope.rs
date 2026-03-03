use std::{
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{
    checker::{Type, TypeResolver},
    lexer::SourceCode,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeType {
    Global,
    Function,
    Closure,
    Block,
    MatchCase,
    Loop,
    Struct(usize),
}

pub struct Scope {
    pub types: TypeResolver,
    pub source: Rc<SourceCode>,
    scope_type: ScopeType,
    parent: Option<Box<Scope>>,
    values: HashMap<String, Type>,
    return_type: Option<Type>,
}

impl Scope {
    pub fn new(source: Rc<SourceCode>, types: TypeResolver) -> Self {
        Self {
            types,
            source,
            scope_type: ScopeType::Global,
            parent: None,
            values: HashMap::new(),
            return_type: None,
        }
    }

    pub fn nest(
        self: Box<Scope>,
        scope_type: ScopeType,
        handler: impl FnOnce(Box<Scope>) -> Box<Scope>,
    ) -> Box<Scope> {
        let (scope, _) = self.nest_with(scope_type, |scope| (handler(scope), ()));
        scope
    }

    pub fn nest_with<T>(
        self: Box<Scope>,
        scope_type: ScopeType,
        handler: impl FnOnce(Box<Scope>) -> (Box<Scope>, T),
    ) -> (Box<Scope>, T) {
        let source = self.source.clone();
        let types = self.types.nest();
        let scope = Box::new(Self {
            scope_type,
            parent: Some(self),
            ..Self::new(source, types)
        });
        let (scope, result) = handler(scope);
        (scope.parent(), result)
    }

    pub fn nest_fn(
        self: Box<Scope>,
        return_type: &Type,
        handler: impl FnOnce(Box<Scope>) -> Box<Scope>,
    ) -> Box<Scope> {
        let source = self.source.clone();
        let types = self.types.nest();
        let scope = Box::new(Self {
            scope_type: ScopeType::Function,
            parent: Some(self),
            return_type: Some(return_type.clone()),
            ..Self::new(source, types)
        });
        handler(scope).parent()
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

    fn parent(self) -> Box<Scope> {
        self.parent.unwrap()
    }

    pub fn add_value(&mut self, identifier: &str, value: Type) {
        self.values.insert(identifier.to_owned(), value);
    }

    pub fn add_value_or(&mut self, identifier: &str, value: Type, if_present: impl Fn(&Scope)) {
        let entry = self.values.entry(identifier.to_owned());
        if let Entry::Vacant(v) = entry {
            v.insert(value);
        } else {
            if_present(self);
        };
    }

    pub fn get_value(&self, identifier: &String) -> Option<Type> {
        self.get_local_value(identifier)
            .or_else(|| self.get_parent_value(identifier))
    }

    pub fn get_local_value(&self, identifier: &String) -> Option<Type> {
        self.values.get(identifier).cloned()
    }

    fn get_parent_value(&self, identifier: &String) -> Option<Type> {
        self.parent
            .as_ref()
            .and_then(|parent| parent.get_value(identifier))
    }

    pub fn get_type_index(&self, identifier: &String) -> Option<usize> {
        self.types.get_index(identifier).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_type_index(identifier))
        })
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        self.types.get_type(index).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_type(index))
        })
    }

    pub fn add_type(&mut self, identifier: &str, alias: Type) {
        self.types.add(identifier, alias);
    }
}
