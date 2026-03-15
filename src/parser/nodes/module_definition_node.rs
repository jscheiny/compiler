use std::rc::Rc;

use crate::{
    checker::{RuntimeType, Scope, Type},
    parser::{EnumNode, FunctionNode, InterfaceNode, NameNode, StructNode, TypeAliasNode},
};

pub struct ExportableModuleDefinitionNode {
    pub public: bool,
    pub definition: ModuleDefinitionNode,
}

pub enum ModuleDefinitionNode {
    Enum(Rc<EnumNode>),
    Function(FunctionNode),
    Interface(Rc<InterfaceNode>),
    Struct(Rc<StructNode>),
    TypeAlias(TypeAliasNode),
}

impl ModuleDefinitionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Enum(node) => node.check(scope),
            Self::Function(node) => node.check(scope),
            Self::Interface(node) => node.check(scope),
            Self::Struct(node) => node.check(scope),
            Self::TypeAlias(node) => node.check(scope).0,
        }
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Type(RuntimeType::Enum(node.get_type(scope)))),
            Self::Function(node) => Some(Type::Function(node.get_type(scope).clone())),
            Self::Struct(node) => Some(Type::Type(RuntimeType::Struct(node.get_type()))),
            // TODO Consider how these are added to scope
            Self::Interface(_) | Self::TypeAlias(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.add_value(self.name(), resolved_type);
        }
    }

    pub fn resolve_type(&mut self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Enum(node.get_type(scope))),
            Self::Interface(node) => Some(Type::Interface(node.get_type(scope))),
            Self::Struct(node) => Some(Type::Struct(node.get_type())),
            Self::TypeAlias(node) => Some(node.get_type(scope, None).clone()),
            Self::Function(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.resolve_type(self.name(), resolved_type);
        }
    }

    pub fn name(&self) -> &NameNode {
        match self {
            Self::Enum(node) => &node.name,
            Self::Function(node) => &node.signature.name,
            Self::Interface(node) => &node.name,
            Self::Struct(node) => &node.name,
            Self::TypeAlias(node) => &node.name,
        }
    }
}
