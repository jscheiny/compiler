use crate::{
    checker::{RuntimeType, Scope, Type, TypeResolver},
    lexer::SourceCode,
    parser::{EnumNode, FunctionNode, Identified, IdentifierNode, Node, StructNode, TypeAliasNode},
};

pub struct ExportableModuleDefinitionNode {
    pub public: bool,
    pub definition: ModuleDefinitionNode,
}

pub enum ModuleDefinitionNode {
    Struct(StructNode),
    Enum(EnumNode),
    Function(FunctionNode),
    TypeAlias(TypeAliasNode),
}

impl ModuleDefinitionNode {
    pub fn check(&self, scope: Box<Scope>) -> Box<Scope> {
        match self {
            Self::Struct(node) => node.check(scope),
            Self::Enum(node) => node.check(scope),
            Self::Function(node) => node.check(scope),
            Self::TypeAlias(node) => {
                node.check();
                scope
            }
        }
    }

    pub fn add_to_scope(&self, scope: &mut Scope) {
        match self {
            Self::Enum(node) => {
                let enum_type =
                    RuntimeType::Enum(node.get_type(&scope.types, &scope.source).clone());
                scope.add(node.id(), Type::Type(enum_type));
            }
            Self::Struct(node) => {
                let struct_type =
                    RuntimeType::Struct(node.get_type(&scope.types, &scope.source).clone());
                scope.add(node.id(), Type::Type(struct_type));
            }
            Self::Function(node) => {
                scope.add(
                    node.id(),
                    Type::Function(node.get_type(&scope.types, &scope.source).clone()),
                );
            }
            // TODO Consider how these are added to scope
            Self::TypeAlias(_) => {}
        }
    }

    pub fn resolve_type(&mut self, types: &mut TypeResolver, source: &SourceCode) {
        match self {
            Self::Struct(node) => {
                let resolved_type = Type::Struct(node.get_type(types, source).clone());
                types.resolve(node.id(), resolved_type);
            }
            Self::Enum(node) => {
                let resolved_type = Type::Enum(node.get_type(types, source).clone());
                types.resolve(node.id(), resolved_type);
            }
            Self::TypeAlias(node) => {
                let resolved_type = node.get_type(types, source).clone();
                types.resolve(node.id(), resolved_type);
            }
            Self::Function(_) => {}
        }
    }

    pub fn identifier(&self) -> &Node<IdentifierNode> {
        match self {
            Self::Struct(node) => &node.identifier,
            Self::Enum(node) => &node.identifier,
            Self::Function(node) => &node.identifier,
            Self::TypeAlias(node) => &node.identifier,
        }
    }
}

impl Identified for ModuleDefinitionNode {
    fn id(&self) -> &String {
        match self {
            Self::Struct(node) => node.id(),
            Self::Enum(node) => node.id(),
            Self::TypeAlias(node) => node.id(),
            Self::Function(node) => node.id(),
        }
    }
}
