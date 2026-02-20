use crate::{
    checker::{EnumType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, MatchPatternNode, Node},
};

pub struct MatchCaseNode {
    pub patterns: Vec<Node<MatchPatternNode>>,
    pub if_match: Node<ExpressionNode>,
}

impl MatchCaseNode {
    pub fn check(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
        subject_type: Option<&EnumType>,
    ) -> (Box<Scope>, Type) {
        // TODO handle pattern checking
        self.if_match.check_expected(types, scope, expected_type)
    }
}
