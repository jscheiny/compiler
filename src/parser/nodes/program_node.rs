use std::rc::Rc;

use crate::{
    checker::{Scope, TypeResolver},
    parser::{ExportableModuleDefinitionNode, Identified, ModuleDefinitionNode, Node},
};

pub struct ProgramNode {
    pub definitions: Vec<Node<ExportableModuleDefinitionNode>>,
}

impl ProgramNode {
    pub fn check(&mut self) {
        let mut types = TypeResolver::new();
        for definition in self.definitions() {
            match definition {
                ModuleDefinitionNode::Enum(_)
                | ModuleDefinitionNode::TypeAlias(_)
                | ModuleDefinitionNode::Struct(_) => types.declare(definition.id()),
                ModuleDefinitionNode::Function(_) => {}
            }
        }

        for definition in self.definitions_mut() {
            definition.resolve_type(&mut types);
        }

        let mut scope = Box::new(self.get_module_scope(types));
        for definition in self.definitions() {
            scope = definition.check(scope);
        }
    }

    pub fn get_module_scope(&self, types: TypeResolver) -> Scope {
        let mut scope = Scope::new(Rc::new(types));
        for definition in self.definitions() {
            definition.add_to_scope(&mut scope);
        }
        scope
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionNode> {
        self.definitions.iter().map(|def| &def.definition)
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut ModuleDefinitionNode> {
        self.definitions.iter_mut().map(|def| &mut def.definition)
    }
}
