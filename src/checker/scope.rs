use std::{collections::HashMap, rc::Rc};

use crate::checker::Type;

#[derive(Default)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    symbols: HashMap<String, Type>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn derive(self: &Rc<Self>) -> Scope {
        Self {
            parent: Some(self.clone()),
            symbols: HashMap::new(),
        }
    }

    pub fn add(&mut self, symbol: &String, value: Type) {
        self.symbols.insert(symbol.clone(), value);
    }

    pub fn contains(&self, symbol: &String) -> bool {
        self.symbols.contains_key(symbol)
    }
}
