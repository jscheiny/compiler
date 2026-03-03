use std::collections::HashMap;

use crate::{
    checker::Type,
    lexer::SourceCode,
    parser::{Identified, IdentifierNode, Node},
};

#[derive(Default, Debug)]
pub struct TypeResolver {
    pub offset: usize,
    types: Vec<Option<Type>>,
    lookup: HashMap<String, usize>,
}

impl TypeResolver {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn nest(&self) -> TypeResolver {
        Self {
            offset: self.offset + self.types.len(),
            ..Self::new()
        }
    }

    pub fn declare(&mut self, identifier: &Node<IdentifierNode>, source: &SourceCode) {
        if self.lookup.contains_key(identifier.id()) {
            source.print_error(
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

    pub fn get_index(&self, identifier: &String) -> Option<usize> {
        self.lookup
            .get(identifier)
            .map(|index| *index + self.offset)
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        if index < self.offset {
            return None;
        }
        self.types.get(index - self.offset).and_then(|t| t.clone())
    }

    pub fn resolve(&mut self, identifier: &String, value: Type) {
        let index = *self.lookup.get(identifier).unwrap();
        if self.types[index].is_none() {
            self.types[index] = Some(value);
        }
    }

    pub fn add(&mut self, identifier: &str, value: Type) {
        self.lookup.insert(identifier.to_owned(), self.types.len());
        self.types.push(Some(value));
    }
}
