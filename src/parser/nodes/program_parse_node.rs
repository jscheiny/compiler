use std::rc::Rc;

use crate::{
    checker::{Scope, TypeResolver},
    parser::{
        ExportableModuleDefinitionParseNode, Identified, ModuleDefinitionParseNode, ParseNode,
    },
};

pub struct ProgramParseNode {
    pub definitions: Vec<ParseNode<ExportableModuleDefinitionParseNode>>,
}

impl ProgramParseNode {
    pub fn check(&mut self) {
        let mut types = TypeResolver::new();
        for definition in self.definitions() {
            types.declare(definition.id());
        }

        for definition in self.definitions_mut() {
            definition.resolve_type(&mut types);
        }

        let _module_scope = self.get_module_scope(&types);
        types.check();
    }

    pub fn get_module_scope(&self, types: &TypeResolver) -> Rc<Scope> {
        let mut scope = Scope::new();
        for definition in self.definitions() {
            scope.add(definition.id(), types.get_type_ref(definition.id()));
        }
        Rc::new(scope)
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionParseNode> {
        self.definitions.iter().map(|def| &def.definition)
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut ModuleDefinitionParseNode> {
        self.definitions.iter_mut().map(|def| &mut def.definition)
    }
}
