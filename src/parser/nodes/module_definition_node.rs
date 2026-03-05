use crate::{
    checker::{RuntimeType, Scope, Type},
    parser::{EnumNode, FunctionNode, Identified, IdentifierNode, Node, StructNode, TypeAliasNode},
};

pub struct ExportableModuleDefinitionNode {
    pub public: bool,
    pub definition: ModuleDefinitionNode,
}

pub enum ModuleDefinitionNode {
    Enum(EnumNode),
    Function(FunctionNode),
    Struct(StructNode),
    TypeAlias(TypeAliasNode),
}

impl ModuleDefinitionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Enum(node) => node.check(scope),
            Self::Function(node) => node.check(scope),
            Self::Struct(node) => node.check(scope),
            Self::TypeAlias(_) => scope, // TODO check for recursion
        }
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Type(RuntimeType::Enum(node.get_type(scope).clone()))),
            Self::Function(node) => Some(Type::Function(node.get_type(scope).clone())),
            Self::Struct(node) => Some(Type::Type(RuntimeType::Struct(
                node.get_type(scope).clone(),
            ))),
            // TODO Consider how these are added to scope
            Self::TypeAlias(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.add_value(self.id(), resolved_type);
        }
    }

    pub fn resolve_type(&mut self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Enum(node.get_type(scope).clone())),
            Self::Struct(node) => Some(Type::Struct(node.get_type(scope).clone())),
            Self::TypeAlias(node) => Some(node.get_type(scope).clone()),
            Self::Function(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.resolve_type(self.id(), resolved_type);
        }
    }

    pub fn identifier(&self) -> &Node<IdentifierNode> {
        match self {
            Self::Enum(node) => &node.identifier,
            Self::Function(node) => &node.identifier,
            Self::Struct(node) => &node.identifier,
            Self::TypeAlias(node) => &node.identifier,
        }
    }
}

impl Identified for ModuleDefinitionNode {
    fn id(&self) -> &String {
        match self {
            Self::Enum(node) => node.id(),
            Self::Function(node) => node.id(),
            Self::Struct(node) => node.id(),
            Self::TypeAlias(node) => node.id(),
        }
    }
}
