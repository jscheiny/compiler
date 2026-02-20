use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
};

use crate::{
    checker::{EnumType, Scope, ScopeType, Type, TypeResolver},
    parser::{EnumVariantNode, Identified, IdentifierNode, MethodNode, Node, NodeVec},
};

pub struct EnumNode {
    identifier: Node<IdentifierNode>,
    variants: NodeVec<EnumVariantNode>,
    methods: Option<NodeVec<MethodNode>>,
    resolved_type: OnceCell<EnumType>,
}

impl EnumNode {
    pub fn new(
        identifier: Node<IdentifierNode>,
        variants: NodeVec<EnumVariantNode>,
        methods: Option<NodeVec<MethodNode>>,
    ) -> Self {
        Self {
            identifier,
            variants,
            methods,
            resolved_type: OnceCell::new(),
        }
    }

    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> Box<Scope> {
        let index = types.get_ref(self.id()).unwrap();
        let mut scope = scope.derive(ScopeType::Struct(index));
        let mut scope_names = HashSet::new();
        for variant in self.variants.iter() {
            if !scope_names.insert(variant.id()) {
                println!("Type error: Duplicate variant of name `{}`", variant.id())
            }
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.iter() {
                if scope_names.contains(method.id()) {
                    println!("Type error: Duplicate member of name `{}`", method.id())
                } else {
                    let method_type = Type::Function(method.function.get_type(types).clone());
                    scope.add(method.id(), method_type);
                    scope_names.insert(method.id());
                }
            }

            for method in methods.iter() {
                scope = method.check(types, scope)
            }
        }

        scope.parent()
    }

    pub fn get_type(&self, types: &mut TypeResolver) -> &EnumType {
        self.resolved_type.get_or_init(|| self.get_type_impl(types))
    }

    pub fn get_type_impl(&self, types: &mut TypeResolver) -> EnumType {
        let mut variants = HashMap::new();
        for variant in self.variants.iter() {
            let identifier = variant.id().clone();
            let variant = variant.get_type(types).cloned();
            variants.entry(identifier).or_insert(variant);
        }

        let mut methods = HashMap::new();
        if let Some(methods_list) = self.methods.as_ref() {
            for method in methods_list.iter() {
                let member = method.resolve_enum_method(types);
                let identifier = method.id().clone();
                methods.entry(identifier).or_insert(member);
            }
        }

        EnumType {
            identifier: self.id().clone(),
            variants,
            methods,
        }
    }
}

impl Identified for EnumNode {
    fn id(&self) -> &String {
        self.identifier.id()
    }
}
