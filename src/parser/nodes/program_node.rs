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
            types.declare(definition.id());
        }

        for definition in self.definitions_mut() {
            definition.resolve_type(&mut types);
        }

        let module_scope = self.get_module_scope(&types);
        for definition in self.definitions() {
            definition.check(&mut types, module_scope.clone());
        }
        types.check();
    }

    pub fn get_module_scope(&self, types: &TypeResolver) -> Rc<Scope> {
        let mut scope = Scope::new();
        for definition in self.definitions() {
            scope.add(definition.id(), types.get_type_ref(definition.id()));
        }
        Rc::new(scope)
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionNode> {
        self.definitions.iter().map(|def| &def.definition)
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut ModuleDefinitionNode> {
        self.definitions.iter_mut().map(|def| &mut def.definition)
    }
}
