use std::{cell::OnceCell, collections::HashMap, rc::Rc};

use crate::{
    checker::{FunctionType, InterfaceType, MemberType, MethodType, Scope, Type, Types},
    parser::{EnumNode, MethodInstanceKind},
};

pub struct EnumType {
    node: Rc<EnumNode>,
    variants: OnceCell<HashMap<String, Option<Type>>>,
    methods: OnceCell<HashMap<String, EnumMethod>>,
}

impl EnumType {
    pub fn from(node: Rc<EnumNode>) -> Rc<EnumType> {
        Rc::new(EnumType {
            node,
            variants: OnceCell::new(),
            methods: OnceCell::new(),
        })
    }

    pub fn complete(&self, scope: &Scope) {
        self.get_variants(scope);
        self.get_methods(scope);
    }

    pub fn name(&self) -> &String {
        &self.node.name
    }

    pub fn get_variant(self: &Rc<Self>, scope: &Scope, name: &String) -> Option<Type> {
        let self_type = Type::Enum(self.clone());
        self.get_variants(scope)
            .get(name)
            .map(|variant_type| match variant_type {
                Some(inner_type) => {
                    Type::Function(FunctionType::simple(inner_type.clone(), self_type))
                }
                None => self_type,
            })
    }

    pub fn get_variants(&self, types: &impl Types) -> &HashMap<String, Option<Type>> {
        self.variants.get_or_init(|| self.init_variants(types))
    }

    fn init_variants(&self, types: &impl Types) -> HashMap<String, Option<Type>> {
        let mut variants = HashMap::new();
        for variant in self.node.variants.iter() {
            let name = variant.name.clone();
            let variant = variant.get_type(types).cloned();
            variants.entry(name).or_insert(variant);
        }

        variants
    }

    pub fn get_method(&self, scope: &Scope, name: &String) -> Option<&EnumMethod> {
        self.get_methods(scope).get(name)
    }

    fn get_methods(&self, scope: &Scope) -> &HashMap<String, EnumMethod> {
        self.methods.get_or_init(|| self.init_methods(scope))
    }

    fn init_methods(&self, scope: &Scope) -> HashMap<String, EnumMethod> {
        let mut methods = HashMap::new();
        if let Some(implementation) = self.node.implementation.as_ref() {
            for method in implementation.get_methods(scope) {
                methods.entry(method.name).or_insert(EnumMethod {
                    public: method.public,
                    method_type: method.method_type,
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
    pub method_type: MethodType,
}

impl MemberType for EnumMethod {
    fn is_public(&self) -> bool {
        self.public
    }

    fn instance_kind(&self) -> MethodInstanceKind {
        self.method_type.instance_kind
    }

    fn get_type(&self) -> Type {
        Type::Function(self.method_type.function_type.clone())
    }
}
