use crate::parser::{
    BlockParseNode, ExpressionParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeDefinitionParseNode,
};

#[derive(Debug)]
pub struct FunctionDefintionParseNode {
    pub identifier: ParseNode<String>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeDefinitionParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

impl Traverse for FunctionDefintionParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        visit(self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse(visit);
        }
        if let Some(return_type) = self.return_type.as_ref() {
            return_type.traverse(visit);
        }
        self.body.traverse(visit);
    }
}

#[derive(Debug)]
pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

impl Traverse for FunctionBodyParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        match self {
            Self::Expression(node) => node.traverse(visit),
            Self::Block(node) => node.traverse(visit),
        }
    }
}

#[derive(Debug)]
pub struct ParameterParseNode {
    pub identifier: ParseNode<String>,
    pub type_def: ParseNode<TypeDefinitionParseNode>,
}

impl Traverse for ParameterParseNode {
    fn traverse(&self, visit: &impl Fn(TokenSpan)) {
        visit(self.identifier.span);
        self.type_def.traverse(visit);
    }
}
