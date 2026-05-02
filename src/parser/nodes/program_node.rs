use std::rc::Rc;

use crate::{
    checker::ModuleScope,
    lexer::SourceCode,
    parser::{ExportableModuleDefinitionNode, ModuleDefinitionNode, Node},
};

pub struct ProgramNode {
    pub definitions: Vec<Node<ExportableModuleDefinitionNode>>,
}

impl ProgramNode {
    pub fn check(&mut self, source: Rc<SourceCode>) {
        let mut scope = ModuleScope::new(source);
        for definition in self.definitions() {
            if let Some(type_node) = definition.to_module_type_node() {
                scope.declare(definition.name(), type_node);
            }
        }

        scope.resolve();

        let mut scope = scope.to_scope();
        for definition in self.definitions_mut() {
            definition.add_to_scope(&mut scope);
        }

        for definition in self.definitions() {
            scope = definition.check(scope);
        }
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionNode> {
        self.definitions.iter().map(|def| &def.definition)
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut ModuleDefinitionNode> {
        self.definitions.iter_mut().map(|def| &mut def.definition)
    }
}
