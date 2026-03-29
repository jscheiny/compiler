use std::collections::HashMap;

use crate::{checker::Type, lexer::SourceCode, parser::NameNode};

#[derive(Default)]
pub struct TypeMap {
    pub offset: usize,
    types: Vec<Option<Type>>,
    lookup: HashMap<String, usize>,
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap::default()
    }

    pub fn nest(&self) -> TypeMap {
        Self {
            offset: self.offset + self.types.len(),
            ..Self::new()
        }
    }

    pub fn declare(&mut self, name: &NameNode, source: &SourceCode) {
        if self.lookup.contains_key(&name.value) {
            source.print_error(
                name.span,
                "Duplicate type name",
                "a type already exists with this name",
            );
            return;
        }

        let index = self.types.len();
        self.types.push(None);
        self.lookup.insert(name.value.clone(), index);
    }

    pub fn get_index(&self, name: &String) -> Option<usize> {
        self.lookup.get(name).map(|index| *index + self.offset)
    }

    pub fn get_type(&self, index: usize) -> Option<Type> {
        if index < self.offset {
            return None;
        }
        self.types.get(index - self.offset).and_then(|t| t.clone())
    }

    pub fn resolve(&mut self, name: &str, value: Type) {
        let index = *self.lookup.get(name).unwrap();
        if self.types[index].is_none() {
            self.types[index] = Some(value);
        }
    }

    pub fn add(&mut self, name: &str, value: Type) {
        self.lookup.insert(name.to_owned(), self.types.len());
        self.types.push(Some(value));
    }
}
