use std::collections::HashMap;

use crate::checker::{DuplicateType, Type, TypeError, TypeReference};

#[derive(Default)]
pub struct TypeResolver {
    types: Vec<Type>,
    lookup: HashMap<String, usize>,
    errors: Vec<TypeError>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_by_id(&self, identifier: &String) -> Option<&Type> {
        let index = self.lookup.get(identifier);
        index.and_then(|index| self.types.get(*index))
    }

    pub fn get_reference(&self, identifier: &String) -> TypeReference {
        let index = self.lookup.get(identifier);
        match index {
            Some(index) => TypeReference::Resolved(*index),
            None => TypeReference::Unresolved,
        }
    }

    pub fn insert(&mut self, identifier: &String, value: Type) {
        let index = self.types.len();
        if self.lookup.contains_key(identifier) {
            self.push_error(TypeError::DuplicateType(DuplicateType {
                identifier: identifier.clone(),
            }));
        } else {
            self.types.push(value);
            self.lookup.insert(identifier.clone(), index);
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
