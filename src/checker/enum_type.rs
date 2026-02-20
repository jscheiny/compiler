use std::collections::HashMap;

use crate::checker::{FunctionType, Type};

#[derive(Default, Clone, Debug)]
pub struct EnumType {
    pub identifier: String,
    pub variants: HashMap<String, Option<Type>>,
    pub methods: HashMap<String, EnumMethod>,
}

impl EnumType {
    pub fn get_variant(&self, identifier: &String) -> Option<Type> {
        // TODO enum type should have a reference index on it to construct its self type here
        let self_type = Type::Enum(self.clone());
        self.variants
            .get(identifier)
            .map(|variant_type| match variant_type {
                Some(inner_type) => {
                    Type::Function(FunctionType::new(inner_type.clone(), self_type))
                }
                None => self_type,
            })
    }
}

#[derive(Clone, Debug)]
pub struct EnumMethod {
    pub public: bool,
    pub function_type: FunctionType,
}
