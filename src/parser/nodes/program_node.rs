use std::rc::Rc;

use crate::{
    checker::{Scope, TypeMap},
    lexer::SourceCode,
    parser::{ExportableModuleDefinitionNode, ModuleDefinitionNode, Node},
};

pub struct ProgramNode {
    pub definitions: Vec<Node<ExportableModuleDefinitionNode>>,
}

impl ProgramNode {
    pub fn check(&mut self, source: Rc<SourceCode>) {
        let mut types = TypeMap::new();
        for definition in self.definitions() {
            if definition.is_type() {
                types.declare(definition.name(), &source);
            }
        }

        let mut scope = self.create_scope(source, types);
        for definition in self.definitions_mut() {
            if definition.is_type() {
                definition.resolve_type(&mut scope);
            }
        }

        for definition in self.definitions() {
            scope = definition.check(scope);
        }
    }

    pub fn create_scope(&self, source: Rc<SourceCode>, types: TypeMap) -> Box<Scope> {
        let mut scope = Scope::new(source, types);
        for definition in self.definitions() {
            definition.add_to_scope(&mut scope);
        }
        Box::new(scope)
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionNode> {
        self.definitions.iter().map(|def| &def.definition)
    }

    fn definitions_mut(&mut self) -> impl Iterator<Item = &mut ModuleDefinitionNode> {
        self.definitions.iter_mut().map(|def| &mut def.definition)
    }
}
