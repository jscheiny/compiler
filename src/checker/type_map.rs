use std::{
    collections::{HashMap, hash_map::Entry},
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::checker::Type;

#[derive(Default)]
pub struct TypeMap {
    lookup: HashMap<String, TypeEntry>,
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap::default()
    }

    pub fn from(lookup: HashMap<String, TypeEntry>) -> Self {
        TypeMap { lookup }
    }

    pub fn get_type_entry(&self, name: &str) -> Option<TypeEntry> {
        self.lookup.get(name).cloned()
    }

    pub fn resolve(&mut self, name: &str, value: Type) {
        match self.lookup.entry(name.to_owned()) {
            Entry::Occupied(mut o) => o.get_mut().value = value,
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
    pub value: Type,
    pub id: usize,
}

// TODO move this
impl TypeEntry {
    pub fn new(value: Type) -> TypeEntry {
        Self {
            value,
            id: new_type_id(),
        }
    }
}

pub fn new_type_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
