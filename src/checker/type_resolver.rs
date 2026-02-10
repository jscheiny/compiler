use std::collections::HashMap;

use crate::checker::{DuplicateType, Scope, Type, TypeError};

#[derive(Default)]
pub struct TypeResolver {
    types: Vec<Option<Type>>,
    lookup: HashMap<String, usize>,
    scopes: Vec<Scope>,
    errors: Vec<TypeError>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn make_scope(&mut self) -> &mut Scope {
        Scope::new(index)
    }

    pub fn declare(&mut self, identifier: &String) {
        if self.lookup.contains_key(identifier) {
            self.push_error(TypeError::DuplicateType(DuplicateType {
                identifier: identifier.clone(),
            }));
            return;
        }

        let index = self.types.len();
        self.types.push(None);
        self.lookup.insert(identifier.clone(), index);
    }

    pub fn get_ref(&self, identifier: &String) -> usize {
        self.lookup[identifier]
    }

    pub fn resolve(&mut self, identifier: &String, value: Type) {
        let index = self.get_ref(identifier);
        if self.types[index].is_none() {
            self.types[index] = Some(value);
        }
    }

    pub fn push_error(&mut self, error: TypeError) {
        self.errors.push(error);
    }

    pub fn check(&self) {
        for error in self.errors.iter() {
            println!("{}", error);
        }
    }
}
