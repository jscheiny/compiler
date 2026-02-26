use std::collections::HashMap;

use crate::{
    checker::Type,
    lexer::SourceCode,
    parser::{Identified, IdentifierNode, Node},
};

#[derive(Default, Debug)]
pub struct TypeResolver {
    types: Vec<Option<Type>>,
    lookup: HashMap<String, usize>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn declare(&mut self, identifier: &Node<IdentifierNode>, source: &SourceCode) {
        if self.lookup.contains_key(identifier.id()) {
            source.print_type_error(
                identifier.span,
                "Duplicate type name",
                "a type already exists with this name",
            );
            return;
        }

        let index = self.types.len();
        self.types.push(None);
        self.lookup.insert(identifier.id().clone(), index);
    }

    pub fn get_ref(&self, identifier: &String) -> Option<usize> {
        self.lookup.get(identifier).copied()
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        self.types[index].clone()
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
}
