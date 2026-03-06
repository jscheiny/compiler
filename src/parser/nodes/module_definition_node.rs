use crate::{
    checker::{RuntimeType, Scope, Type},
    parser::{
        EnumNode, FunctionNode, Identified, IdentifierNode, InterfaceNode, Node, StructNode,
        TypeAliasNode,
    },
};

pub struct ExportableModuleDefinitionNode {
    pub public: bool,
    pub definition: ModuleDefinitionNode,
}

pub enum ModuleDefinitionNode {
    Enum(EnumNode),
    Function(FunctionNode),
    Interface(InterfaceNode),
    Struct(StructNode),
    TypeAlias(TypeAliasNode),
}

impl ModuleDefinitionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Enum(node) => node.check(scope),
            Self::Function(node) => node.check(scope),
            Self::Interface(node) => node.check(scope),
            Self::Struct(node) => node.check(scope),
            Self::TypeAlias(_) => scope, // TODO check for recursion
        }
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Type(RuntimeType::Enum(node.get_type(scope)))),
            Self::Function(node) => Some(Type::Function(node.get_type(scope).clone())),
            Self::Struct(node) => Some(Type::Type(RuntimeType::Struct(node.get_type(scope)))),
            // TODO Consider how these are added to scope
            Self::Interface(_) | Self::TypeAlias(_) => None,
        };

        if let Some(resolved_type) = resolved_type {
            scope.add_value(self.id(), resolved_type);
        }
    }

    pub fn resolve_type(&mut self, scope: &mut Scope) {
        let resolved_type = match self {
            Self::Enum(node) => Some(Type::Enum(node.get_type(scope))),
            Self::Interface(node) => Some(Type::Interface(node.get_type(scope))),
            Self::Struct(node) => Some(Type::Struct(node.get_type(scope))),
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
            Self::Function(node) => &node.signature.identifier,
            Self::Interface(node) => &node.identifier,
            Self::Struct(node) => &node.identifier,
            Self::TypeAlias(node) => &node.identifier,
        }
    }
}

impl Identified for ModuleDefinitionNode {
    fn id(&self) -> &String {
        self.identifier().id()
    }
}
