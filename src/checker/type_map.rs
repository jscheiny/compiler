use std::{
    collections::{HashMap, hash_map::Entry},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::{checker::Type, lexer::SourceCode, parser::NameNode};

#[derive(Default)]
pub struct TypeMap {
    lookup: HashMap<String, TypeEntry>,
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap::default()
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

        self.lookup.insert(name.value.clone(), TypeEntry::empty());
    }

    pub fn get_type_entry(&self, name: &String) -> Option<TypeEntry> {
        self.lookup.get(name).cloned()
    }

    pub fn resolve(&mut self, name: &str, value: Type) {
        match self.lookup.entry(name.to_owned()) {
            Entry::Occupied(mut o) => o.get_mut().fill(value),
            Entry::Vacant(v) => {
                v.insert(TypeEntry::new(value));
            }
        };
    }

    pub fn add(&mut self, name: &str, value: Type) {
        self.lookup.insert(name.to_owned(), TypeEntry::new(value));
    }
}

#[derive(Clone)]
pub struct TypeEntry {
    pub value: Option<Type>,
    pub id: usize,
}

// TODO move this
impl TypeEntry {
    pub fn empty() -> TypeEntry {
        Self {
            value: None,
            id: new_type_id(),
        }
    }

    pub fn new(value: Type) -> TypeEntry {
        Self {
            value: Some(value),
            id: new_type_id(),
        }
    }

    pub fn fill(&mut self, value: Type) {
        if self.value.is_none() {
            self.value = Some(value)
        }
    }
}

pub fn new_type_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
