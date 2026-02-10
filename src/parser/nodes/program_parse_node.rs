use std::rc::Rc;

use crate::{
    checker::{Scope, Type, TypeResolver},
    parser::{
        ExportableModuleDefinitionParseNode, ModuleDefinitionParseNode, ParseNode, TokenSpan,
        Traverse,
    },
};

pub struct ProgramParseNode {
    pub definitions: Vec<ParseNode<ExportableModuleDefinitionParseNode>>,
}

impl Traverse for ProgramParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for definition in self.definitions.iter() {
            definition.traverse("Program.definition", visit);
        }
    }
}

impl ProgramParseNode {
    pub fn check(&self) {
        let mut types = TypeResolver::new();
        for definition in self.definitions() {
            definition.declare_type(&mut types);
        }

        for definition in self.definitions() {
            definition.resolve_type(&mut types);
        }

        let _module_scope = self.get_module_scope(&types);
        types.check();
    }

    pub fn get_module_scope(&self, types: &TypeResolver) -> Rc<Scope> {
        let mut scope = Scope::new();
        for definition in self.definitions() {
            scope.add(
                definition.identifier(),
                Type::Reference(types.get_ref(definition.identifier())),
            );
        }
        Rc::new(scope)
    }

    fn definitions(&self) -> impl Iterator<Item = &ModuleDefinitionParseNode> {
        self.definitions.iter().map(|def| &def.value.definition)
    }
}
