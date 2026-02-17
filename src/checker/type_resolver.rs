use std::collections::HashMap;

use crate::checker::{DuplicateType, Type, TypeError};

#[derive(Default)]
pub struct TypeResolver {
    types: Vec<Option<Type>>,
    lookup: HashMap<String, usize>,
    errors: Vec<TypeError>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Default::default()
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

    pub fn get_ref(&self, identifier: &String) -> Option<usize> {
        self.lookup.get(identifier).copied()
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        self.types[index].clone()
    }

    pub fn get_type_by_ref(&self, identifier: &String) -> Option<Type> {
        self.get_ref(identifier)
            .and_then(|index| self.get_type(index))
    }

    pub fn resolve(&mut self, identifier: &String, value: Type) {
        let index = self.get_ref(identifier);
        if let Some(index) = index {
            if self.types[index].is_none() {
                self.types[index] = Some(value);
            }
        } else {
            panic!("Could not resolve {}", identifier);
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
