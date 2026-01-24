use crate::parser::{
    BlockParseNode, ExpressionParseNode, ParseNode, ParseNodeVec, TokenSpan, Traverse,
    TypeParseNode,
};

#[derive(Debug)]
pub struct FunctionDefintionParseNode {
    pub identifier: ParseNode<String>,
    pub parameters: ParseNodeVec<ParameterParseNode>,
    pub return_type: Option<ParseNode<TypeParseNode>>,
    pub body: ParseNode<FunctionBodyParseNode>,
}

impl Traverse for FunctionDefintionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("FunctionDefinition.identifier", self.identifier.span);
        visit("FunctionDefinition.parameters", self.parameters.span);
        for parameter in self.parameters.value.iter() {
            parameter.traverse("FunctionDefintion.parameter", visit);
        }
        if let Some(return_type) = self.return_type.as_ref() {
            return_type.traverse("FunctionDefinition.return", visit);
        }
        self.body.traverse("FunctionDefinition.body", visit);
    }
}

#[derive(Debug)]
pub enum FunctionBodyParseNode {
    Expression(ExpressionParseNode),
    Block(BlockParseNode),
}

impl Traverse for FunctionBodyParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        match self {
            Self::Expression(node) => node.traverse(visit),
            Self::Block(node) => node.traverse(visit),
        }
    }
}

#[derive(Debug)]
pub struct ParameterParseNode {
    pub identifier: ParseNode<String>,
    pub type_def: ParseNode<TypeParseNode>,
}

impl Traverse for ParameterParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Parameter.identifier", self.identifier.span);
        self.type_def.traverse("Parameter.type", visit);
    }
}
