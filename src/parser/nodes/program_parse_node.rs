use crate::{
    checker::TypeResolver,
    parser::{ExportableModuleDefinitionParseNode, ParseNode, TokenSpan, Traverse},
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
        for definition in self.definitions.iter() {
            definition.value.definition.declare_type(&mut types);
        }
        for definition in self.definitions.iter() {
            definition.value.definition.resolve_type(&mut types);
        }
        types.check();
    }
}
