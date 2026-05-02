use std::{collections::HashMap, rc::Rc};

use crate::{
    checker::{Scope, Type, TypeEntry, TypeMap, Types, new_type_id},
    lexer::SourceCode,
    parser::{EnumNode, InterfaceNode, NameNode, StructNode, TokenSpan, TypeAliasNode},
};

pub enum ModuleTypeNode {
    Enum(Rc<EnumNode>),
    Interface(Rc<InterfaceNode>),
    Struct(Rc<StructNode>),
    TypeAlias(Rc<TypeAliasNode>),
}

impl ModuleTypeNode {
    pub fn get_type(&self, scope: &ModuleScope) -> Type {
        match self {
            ModuleTypeNode::Enum(node) => Type::Enum(node.get_type(scope)),
            ModuleTypeNode::Interface(node) => Type::Interface(node.get_type(scope)),
            ModuleTypeNode::Struct(node) => Type::Struct(node.get_type(scope)),
            ModuleTypeNode::TypeAlias(node) => node.get_type(scope).clone(),
        }
    }
}

struct ModuleTypeEntry {
    node: ModuleTypeNode,
    id: usize,
}

impl ModuleTypeEntry {
    pub fn to_type_entry(&self, scope: &ModuleScope) -> TypeEntry {
        TypeEntry {
            value: self.node.get_type(scope),
            id: self.id,
        }
    }
}

pub struct ModuleScope {
    source: Rc<SourceCode>,
    lookup: HashMap<String, ModuleTypeEntry>,
}

impl ModuleScope {
    pub fn new(source: Rc<SourceCode>) -> Self {
        ModuleScope {
            source,
            lookup: HashMap::new(),
        }
    }

    pub fn to_scope(self) -> Box<Scope> {
        let types = self
            .lookup
            .iter()
            .map(|(key, value)| (key.clone(), value.to_type_entry(&self)))
            .collect();
        Box::new(Scope::new(self.source.clone(), TypeMap::from(types)))
    }

    pub fn declare(&mut self, name: &NameNode, node: ModuleTypeNode) {
        if self.lookup.contains_key(&name.value) {
            self.source.print_error(
                name.span,
                "Duplicate type name",
                "a type already exists with this name",
            );
            return;
        }

        self.lookup.insert(
            name.value.clone(),
            ModuleTypeEntry {
                node,
                id: new_type_id(),
            },
        );
    }

    pub fn resolve(&self) {
        for entry in self.lookup.values() {
            entry.node.get_type(self);
        }
    }
}

impl Types for ModuleScope {
    fn get_type_id(&self, name: &str) -> Option<usize> {
        self.lookup.get(name).map(|entry| entry.id)
    }

    fn get_type(&self, name: &str) -> Option<Type> {
        self.lookup.get(name).map(|entry| entry.node.get_type(self))
    }

    fn get_return_type(&self) -> Option<Type> {
        None
    }

    fn get_self_type(&self) -> Option<Type> {
        None
    }

    fn print_error(&self, span: TokenSpan, message: &str, inline_message: &str) {
        self.source.print_error(span, message, inline_message);
    }
}
