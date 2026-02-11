use std::collections::HashMap;

use crate::checker::Type;

#[derive(Default)]
pub struct Scope {
    // parent: Option<&'a Box<Scope<'a>>>,
    values: HashMap<String, Type>,
    types: HashMap<String, Type>,
    self_values: Option<HashMap<String, Type>>,
}

impl Scope {
    pub fn new() -> Self {
        Default::default()
    }

    // pub fn derive<'a, 'b>(self: &'a Scope<'b>) -> Scope<'a + 'b> {
    //     Self {
    //         parent: Some(self),
    //         ..Self::new()
    //     }
    // }

    pub fn add_type(&mut self, symbol: &String, value: Type) {
        self.types.insert(symbol.clone(), value);
    }

    pub fn add_value(&mut self, symbol: &String, value: Type) {
        self.values.insert(symbol.clone(), value);
    }

    pub fn contains_type(&self, symbol: &String) -> bool {
        self.types.contains_key(symbol)
        // || self
        //     .parent
        //     .as_ref()
        //     .map(|parent| parent.contains_type(symbol))
        //     .unwrap_or(false)
    }

    pub fn contains_value(&self, symbol: &String) -> bool {
        self.values.contains_key(symbol)
        // || self
        //     .parent
        //     .as_ref()
        //     .map(|parent| parent.contains_type(symbol))
        //     .unwrap_or(false)
    }
}
