use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
};

use crate::{
    checker::{EnumType, Scope, ScopeType, Type, TypeResolver},
    lexer::SourceCode,
    parser::{EnumVariantNode, Identified, IdentifierNode, MethodNode, Node, NodeVec},
};

pub struct EnumNode {
    pub identifier: Node<IdentifierNode>,
    pub variants: NodeVec<EnumVariantNode>,
    pub methods: Option<NodeVec<MethodNode>>,
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

    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        let index = scope.types.get_ref(self.id()).unwrap();
        scope.nest(ScopeType::Struct(index), |mut scope| {
            let mut scope_names = HashSet::new();
            for variant in self.variants.iter() {
                if !scope_names.insert(variant.id()) {
                    scope.source.print_type_error(
                        variant.identifier.span,
                        &format!("Duplicate enum variant `{}`", variant.id()),
                        &format!("a variant of `{}` already exists with this name", self.id()),
                    );
                }
            }

            if let Some(methods) = self.methods.as_ref() {
                for method in methods.iter() {
                    if scope_names.contains(method.id()) {
                        scope.source.print_type_error(
                            method.function.identifier.span,
                            &format!("Duplicate enum member `{}`", method.id()),
                            &format!(
                                "a variant or method of `{}` already exists with this name",
                                self.id()
                            ),
                        );
                    } else {
                        let method_type = Type::Function(
                            method
                                .function
                                .get_type(&scope.types, &scope.source)
                                .clone(),
                        );
                        scope.add(method.id(), method_type);
                        scope_names.insert(method.id());
                    }
                }

                for method in methods.iter() {
                    scope = method.check(scope)
                }
            }

            scope
        })
    }

    pub fn get_type(&self, types: &TypeResolver, source: &SourceCode) -> &EnumType {
        self.resolved_type
            .get_or_init(|| self.get_type_impl(types, source))
    }

    pub fn get_type_impl(&self, types: &TypeResolver, source: &SourceCode) -> EnumType {
        let mut variants = HashMap::new();
        for variant in self.variants.iter() {
            let identifier = variant.id().clone();
            let variant = variant.get_type(types, source).cloned();
            variants.entry(identifier).or_insert(variant);
        }

        let mut methods = HashMap::new();
        if let Some(methods_list) = self.methods.as_ref() {
            for method in methods_list.iter() {
                let member = method.resolve_enum_method(types, source);
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
