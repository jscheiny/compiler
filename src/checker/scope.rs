use std::{
    collections::{HashMap, hash_map::Entry},
    rc::Rc,
};

use crate::{
    checker::{Type, TypeMap},
    lexer::{EnumToken, Keyword, SourceCode},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScopeType {
    Global,
    Function,
    Closure,
    Block,
    MatchCase,
    Loop,
    Type,
    Struct(usize),
}

pub struct Scope {
    pub source: Rc<SourceCode>,
    scope_type: ScopeType,
    parent: Option<Box<Scope>>,
    values: HashMap<String, Type>,
    types: TypeMap,
    return_type: Option<Type>,
}

impl Scope {
    pub fn new(source: Rc<SourceCode>, types: TypeMap) -> Self {
        Self {
            source,
            scope_type: ScopeType::Global,
            parent: None,
            values: HashMap::new(),
            types,
            return_type: None,
        }
    }

    pub fn nest(
        self: Box<Scope>,
        scope_type: ScopeType,
        handler: impl FnOnce(Box<Scope>) -> Box<Scope>,
    ) -> Box<Scope> {
        let (scope, ()) = self.nest_with(scope_type, |scope| (handler(scope), ()));
        scope
    }

    pub fn nest_with<T>(
        self: Box<Scope>,
        scope_type: ScopeType,
        handler: impl FnOnce(Box<Scope>) -> (Box<Scope>, T),
    ) -> (Box<Scope>, T) {
        let source = self.source.clone();
        let types = self.types.nest();
        let mut scope = Box::new(Self {
            scope_type,
            parent: Some(self),
            ..Self::new(source, types)
        });
        if let ScopeType::Struct(index) = scope_type {
            let self_type = Type::Reference(index).as_deref(&scope);
            scope.add_type_and_value(Keyword::SelfType.as_str(), &self_type);
        }
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
        let mut scope = Box::new(Self {
            scope_type: ScopeType::Function,
            parent: Some(self),
            return_type: Some(return_type.clone()),
            ..Self::new(source, types)
        });
        scope.add_type_and_value(Keyword::Result.as_str(), return_type);
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
                .is_some_and(|parent| parent.within(scope_type))
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

    pub fn global(&self) -> &Scope {
        match self.parent.as_ref() {
            Some(parent) => parent.global(),
            None => self,
        }
    }

    fn parent(self) -> Box<Scope> {
        self.parent.unwrap()
    }

    pub fn add_value(&mut self, name: &str, value: Type) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn add_value_or(&mut self, name: &str, value: Type, if_present: impl Fn(&Scope)) {
        let entry = self.values.entry(name.to_owned());
        if let Entry::Vacant(v) = entry {
            v.insert(value);
        } else {
            if_present(self);
        }
    }

    pub fn get_value(&self, name: &String) -> Option<Type> {
        self.get_local_value(name)
            .or_else(|| self.get_parent_value(name))
    }

    pub fn get_local_value(&self, name: &String) -> Option<Type> {
        self.values.get(name).cloned()
    }

    fn get_parent_value(&self, name: &String) -> Option<Type> {
        self.parent
            .as_ref()
            .and_then(|parent| parent.get_value(name))
    }

    pub fn get_type_index(&self, name: &String) -> Option<usize> {
        self.types.get_index(name).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_type_index(name))
        })
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        self.types.get_type(index).or_else(|| {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_type(index))
        })
    }

    pub fn get_self_type(&self) -> Option<Type> {
        if let ScopeType::Struct(index) = self.scope_type {
            self.get_type(index)
        } else {
            self.parent
                .as_ref()
                .and_then(|parent| parent.get_self_type())
        }
    }

    pub fn add_type(&mut self, name: &str, alias: Type) {
        self.types.add(name, alias);
    }

    pub fn resolve_type(&mut self, name: &str, value: Type) {
        self.types.resolve(name, value);
    }

    fn add_type_and_value(&mut self, name: &str, value: &Type) {
        self.add_type(name, value.clone());
        if let Type::Struct(struct_type) = value.deref(self) {
            self.add_value(name, Type::Function(struct_type.get_constructor(self)));
        }
    }
}
