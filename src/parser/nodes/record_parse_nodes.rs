use crate::parser::{
    FunctionDefintionParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<String>,
    pub member_list: ParseNodeVec<RecordMemberParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for RecordDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        visit(self.member_list.span);
        for member in self.member_list.value.iter() {
            member.traverse(visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit(methods.span);
            for method in methods.value.iter() {
                method.traverse(visit);
            }
        }
    }
}

#[derive(Debug)]
pub enum RecordType {
    Structure,
    Tuple,
}

#[derive(Debug)]
pub struct RecordMemberParseNode {
    pub public: bool,
    pub identifier: ParseNode<String>,
    pub type_def: ParseNode<TypeDefinitionParseNode>,
}

impl Traverse for RecordMemberParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        self.type_def.traverse(visit);
    }
}

#[derive(Debug)]
pub struct MethodParseNode {
    pub public: bool,
    pub function: ParseNode<FunctionDefintionParseNode>,
}

impl Traverse for MethodParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        self.function.traverse(visit);
    }
}
