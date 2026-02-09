use crate::parser::{ExportableModuleDefinitionParseNode, ParseNode, TokenSpan, Traverse};

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
