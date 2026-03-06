use std::{collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{EnumNode, Identified},
};

pub struct EnumType {
    node: Rc<EnumNode>,
    pub variants: HashMap<String, Option<Type>>,
    pub methods: HashMap<String, EnumMethod>,
}

impl EnumType {
    pub fn from(node: Rc<EnumNode>, scope: &Scope) -> Rc<EnumType> {
        let mut variants = HashMap::new();
        for variant in node.variants.iter() {
            let identifier = variant.id().clone();
            let variant = variant.get_type(scope).cloned();
            variants.entry(identifier).or_insert(variant);
        }

        let mut methods = HashMap::new();
        if let Some(implementation) = node.implementation.as_ref() {
            for (identifier, public, function_type) in implementation.get_methods(scope) {
                methods.entry(identifier).or_insert(EnumMethod {
                    public,
                    function_type,
                });
            }
        }

        Rc::new(EnumType {
            variants,
            methods,
            node,
        })
    }

    pub fn id(&self) -> &String {
        self.node.identifier.id()
    }

    pub fn get_variant(self: &Rc<Self>, identifier: &String) -> Option<Type> {
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

pub struct EnumMethod {
    pub public: bool,
    pub function_type: Rc<FunctionType>,
}
