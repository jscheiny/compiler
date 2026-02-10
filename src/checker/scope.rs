use std::collections::HashMap;

use crate::checker::Type;

#[derive(Default)]
pub struct Scope {
    index: usize,
    parent: Option<usize>,
    symbols: HashMap<String, Type>,
}

impl Scope {
    pub fn new(index: usize) -> Self {
        Self {
            index,
            parent: None,
            symbols: HashMap::new(),
        }
    }

    // pub fn derive(index)

    pub fn add(&mut self, symbol: &String, value: Type) {
        self.symbols.insert(symbol.clone(), value);
    }

    pub fn contains(&self, symbol: &String) -> bool {
        self.symbols.contains_key(symbol)
    }
}
