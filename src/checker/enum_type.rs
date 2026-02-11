use std::collections::HashMap;

use crate::checker::{DuplicateMemberName, FunctionType, Type, TypeError, TypeResolver};

#[derive(Default, Clone, Debug)]
pub struct EnumType {
    pub variants: HashMap<String, Option<Type>>,
    pub methods: HashMap<String, EnumMethod>,
}

impl EnumType {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_variant(
        &mut self,
        identifier: &String,
        container_name: &str,
        variant: Option<Type>,
        types: &mut TypeResolver,
    ) {
        if self.variants.contains_key(identifier) {
            types.push_error(TypeError::DuplicateMemberName(DuplicateMemberName {
                container_name: container_name.to_owned(),
                container_type: "enum".to_owned(),
                member_name: identifier.clone(),
                member_type: "variant".to_owned(),
            }));
        } else {
            self.variants.insert(identifier.clone(), variant);
        }
    }

    pub fn add_method(
        &mut self,
        identifier: &String,
        container_name: &str,
        method: EnumMethod,
        types: &mut TypeResolver,
    ) {
        if self.methods.contains_key(identifier) {
            types.push_error(TypeError::DuplicateMemberName(DuplicateMemberName {
                container_name: container_name.to_owned(),
                container_type: "enum".to_owned(),
                member_name: identifier.clone(),
                member_type: "method".to_owned(),
            }));
        } else {
            self.methods.insert(identifier.clone(), method);
        }
    }
}

#[derive(Clone, Debug)]
pub struct EnumMethod {
    pub public: bool,
    pub function_type: FunctionType,
}
