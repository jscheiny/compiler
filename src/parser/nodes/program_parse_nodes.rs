use crate::parser::{ModuleDefinition, ParseNode, TokenSpan, Traverse};

pub struct ProgramParseNode {
    pub definitions: Vec<ParseNode<ExportableModuleDefinition>>,
}

impl Traverse for ProgramParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        for definition in self.definitions.iter() {
            definition.traverse("Program.definition", visit);
        }
    }
}

pub struct ExportableModuleDefinition {
    pub public: bool,
    pub definition: ModuleDefinition,
}

impl Traverse for ExportableModuleDefinition {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        self.definition.traverse(visit);
    }
}
