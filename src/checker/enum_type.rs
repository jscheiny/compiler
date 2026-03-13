use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type},
    parser::{EnumNode, Named},
};

pub struct EnumType {
    node: Rc<EnumNode>,
    pub variants: HashMap<String, Option<Type>>,
    methods: OnceCell<HashMap<String, EnumMethod>>,
}

impl EnumType {
    pub fn from(node: Rc<EnumNode>, scope: &Scope) -> Rc<EnumType> {
        let mut variants = HashMap::new();
        for variant in node.variants.iter() {
            let identifier = variant.name().clone();
            let variant = variant.get_type(scope).cloned();
            variants.entry(identifier).or_insert(variant);
        }

        Rc::new(EnumType {
            node,
            variants,
            methods: OnceCell::new(),
        })
    }

    pub fn name(&self) -> &String {
        self.node.name.name()
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

    pub fn get_method(&self, scope: &Scope, identifier: &String) -> Option<&EnumMethod> {
        self.methods
            .get_or_init(|| self.init_methods(scope))
            .get(identifier)
    }

    fn init_methods(&self, scope: &Scope) -> HashMap<String, EnumMethod> {
        let scope = scope.global();
        let mut methods = HashMap::new();
        if let Some(implementation) = self.node.implementation.as_ref() {
            for (identifier, public, function_type) in implementation.get_methods(scope) {
                methods.entry(identifier).or_insert(EnumMethod {
                    public,
                    function_type,
                });
            }
        }

        methods
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        self.node
            .implementation
            .as_ref()
            .map(|implementation| implementation.implements(scope, interface_type))
            .unwrap_or(false)
    }
}

pub struct EnumMethod {
    pub public: bool,
    pub function_type: Rc<FunctionType>,
}
