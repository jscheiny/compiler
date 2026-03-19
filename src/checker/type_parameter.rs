use std::{
    collections::HashMap,
    fmt::Display,
    hash::Hash,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::checker::{Type, TypeParameterBindings};

pub type TypeParameterMap = HashMap<String, Rc<TypeParameter>>;

#[derive(Eq)]
pub struct TypeParameter {
    pub name: String,
    id: usize,
}

impl TypeParameter {
    pub fn new(name: String) -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        Self {
            name,
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn bind(self: &Rc<Self>, bindings: &TypeParameterBindings) -> Type {
        bindings
            .get(self)
            .cloned()
            .unwrap_or_else(|| Type::TypeParameter(self.clone()))
    }
}

impl PartialEq for TypeParameter {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for TypeParameter {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Display for TypeParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
