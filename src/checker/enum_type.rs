use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, Scope, Type, Types},
    parser::EnumNode,
};

pub struct EnumType {
    node: Rc<EnumNode>,
    pub variants: HashMap<String, Option<Type>>,
    methods: OnceCell<HashMap<String, EnumMethod>>,
}

impl EnumType {
    pub fn from(node: Rc<EnumNode>, types: &impl Types) -> Rc<EnumType> {
        let mut variants = HashMap::new();
        for variant in node.variants.iter() {
            let name = variant.name.clone();
            let variant = variant.get_type(types).cloned();
            variants.entry(name).or_insert(variant);
        }

        Rc::new(EnumType {
            node,
            variants,
            methods: OnceCell::new(),
        })
    }

    pub fn name(&self) -> &String {
        &self.node.name
    }

    pub fn get_variant(self: &Rc<Self>, name: &String) -> Option<Type> {
        let self_type = Type::Enum(self.clone());
        self.variants
            .get(name)
            .map(|variant_type| match variant_type {
                Some(inner_type) => {
                    Type::Function(FunctionType::simple(inner_type.clone(), self_type))
                }
                None => self_type,
            })
    }

    pub fn get_method(&self, scope: &Scope, name: &String) -> Option<&EnumMethod> {
        self.methods
            .get_or_init(|| self.init_methods(scope))
            .get(name)
    }

    fn init_methods(&self, scope: &Scope) -> HashMap<String, EnumMethod> {
        let scope = scope.global();
        let mut methods = HashMap::new();
        if let Some(implementation) = self.node.implementation.as_ref() {
            for method in implementation.get_methods(scope) {
                methods.entry(method.name).or_insert(EnumMethod {
                    public: method.public,
                    function_type: method.function_type,
                });
            }
        }

        methods
    }

    pub fn implements(&self, scope: &Scope, interface_type: &Rc<InterfaceType>) -> bool {
        self.node
            .implementation
            .as_ref()
            .is_some_and(|implementation| implementation.implements(scope, interface_type))
    }
}

pub struct EnumMethod {
    pub public: bool,
    pub function_type: Rc<FunctionType>,
}
