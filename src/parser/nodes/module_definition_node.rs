use std::rc::Rc;

use crate::{
    checker::{ModuleTypeNode, Scope, Type},
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
    TypeAlias(Rc<TypeAliasNode>),
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
            Self::Function(node) => Some(Type::Function(node.get_type(scope).clone())),
            // TODO respect the privacy of the constructor
            Self::Struct(node) => Some(Type::Function(node.get_type(scope).get_constructor(scope))),
            Self::Enum(_) | Self::Interface(_) | Self::TypeAlias(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.add_value(self.name(), resolved_type);
        }
    }

    pub fn to_module_type_node(&self) -> Option<ModuleTypeNode> {
        match self {
            Self::Enum(node) => Some(ModuleTypeNode::Enum(node.clone())),
            Self::Interface(node) => Some(ModuleTypeNode::Interface(node.clone())),
            Self::Struct(node) => Some(ModuleTypeNode::Struct(node.clone())),
            Self::TypeAlias(node) => Some(ModuleTypeNode::TypeAlias(node.clone())),
            Self::Function(_) => None,
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
