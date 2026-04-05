use crate::{
    checker::{Scope, ScopeType, Type},
    lexer::{EnumToken, Keyword},
    parser::{
        ArrayExpressionNode, BinaryOpExpressionNode, BlockNode, ClosureExpressionNode,
        ClosureParameterExpressionNode, DeferredMemberExpressionNode, FunctionCallExpressionNode,
        IfExpressionNode, MatchNode, MemberTypeExpressionNode, MemberValueExpressionNode, NameNode,
        PostfixOpExpressionNode, PrefixOpExpressionNode, PrimitiveType, SpreadNode, TokenSpan,
        TupleExpressionNode, TypeBindingExpressionNode,
    },
};

pub enum ExpressionNode {
    Array(ArrayExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    Block(BlockNode),
    BooleanLiteral(bool),
    CharacterLiteral(String),
    Closure(ClosureExpressionNode),
    ClosureParameter(ClosureParameterExpressionNode),
    DeferredMember(DeferredMemberExpressionNode),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    IntegerLiteral(i64),
    Match(MatchNode),
    MemberType(MemberTypeExpressionNode),
    MemberValue(MemberValueExpressionNode),
    Name(NameNode),
    PostfixOp(PostfixOpExpressionNode),
    PrefixOp(PrefixOpExpressionNode),
    SelfRef(NameNode),
    SelfValue(TokenSpan),
    Spread(SpreadNode),
    StringLiteral(String),
    Tuple(TupleExpressionNode),
    TypeBinding(TypeBindingExpressionNode),
    Error,
}

impl ExpressionNode {
    pub fn check(&self, scope: Box<Scope>) -> (Box<Scope>, Type) {
        self.check_expected(scope, None)
    }

    pub fn check_expected(
        &self,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        match self {
            Self::Array(node) => node.check(scope, expected_type),
            Self::BinaryOp(node) => node.check(scope, expected_type),
            Self::Block(node) => {
                let (scope, resolved_type) = node.check(scope, expected_type);
                (scope, resolved_type.unwrap_or(Type::Void))
            }
            Self::BooleanLiteral(_) => (scope, Type::Primitive(PrimitiveType::Bool)),
            Self::CharacterLiteral(_) => (scope, Type::Primitive(PrimitiveType::Char)),
            Self::Closure(node) => node.check(scope, expected_type),
            Self::ClosureParameter(node) => node.check(scope),
            Self::DeferredMember(node) => node.check(scope, expected_type),
            Self::FunctionCall(node) => node.check(scope, expected_type),
            Self::IfExpression(node) => node.check(scope, expected_type),
            Self::IntegerLiteral(_) => (scope, Type::Primitive(PrimitiveType::Int)),
            Self::Match(node) => node.check(scope, expected_type),
            Self::MemberType(node) => node.check(scope),
            Self::MemberValue(node) => node.check(scope, expected_type),
            Self::Name(node) => node.check(scope, expected_type),
            Self::PostfixOp(node) => node.check(scope),
            Self::PrefixOp(node) => node.check(scope),
            Self::SelfRef(name) => check_self_ref(scope, name),
            Self::SelfValue(span) => check_self_value(scope, *span),
            Self::Spread(node) => node.check_invalid(scope, expected_type),
            Self::StringLiteral(_) => (
                scope,
                Type::Array(Box::new(Type::Primitive(PrimitiveType::Char))),
            ),
            Self::Tuple(node) => node.check(scope, expected_type),
            Self::TypeBinding(node) => node.check(scope),
            Self::Error => (scope, Type::Error),
        }
    }

    pub fn check_type(&self, scope: Box<Scope>, span: TokenSpan) -> (Box<Scope>, Type) {
        let ExpressionNode::Name(name) = self else {
            let (scope, _) = self.check(scope);
            scope.source.print_error(
                span,
                "Cannot use type member operator on an expression",
                "must be a type",
            );
            return (scope, Type::Error);
        };

        let Some(type_index) = scope.get_type_index(name) else {
            print_unknown_type_error(&scope, span, name);
            return (scope, Type::Error);
        };

        let result_type = Type::Reference(type_index).as_deref(&scope);
        (scope, result_type)
    }
}

fn print_unknown_type_error(scope: &Scope, span: TokenSpan, name: &str) {
    if name == Keyword::Result.as_str() {
        scope.source.print_error(
            span,
            "`Result` type not available outside of function bodies",
            "cannot use type `Result` here",
        );
    } else if name == Keyword::SelfType.as_str() {
        scope.source.print_error(
            span,
            "`Self` type not available outside of struct or enum",
            "cannot use type `Self` here",
        );
    } else {
        scope.source.print_error(
            span,
            &format!("Unknown type `{name}`"),
            "could not find a type with this name",
        );
    }
}

fn check_self_ref(scope: Box<Scope>, name: &NameNode) -> (Box<Scope>, Type) {
    // TODO maybe replace with using get_self_type and get_field from member_value_expression_node.rs
    let self_scope = scope.find_scope(|scope_type| matches!(scope_type, ScopeType::Struct(_)));
    if let Some(self_scope) = self_scope {
        let resolved_type = self_scope.get_local_value(name);
        if let Some(resolved_type) = resolved_type {
            return (scope, resolved_type);
        }
        scope.source.print_error(
            name.span,
            &format!("Could not find member `{name}`"),
            "self type does not contain a member with this name",
        );
    } else {
        scope.source.print_error(
            name.span.before(),
            "Self reference outside of struct or enum",
            "operator invalid outside of struct or enum",
        );
    }

    (scope, Type::Error)
}

fn check_self_value(scope: Box<Scope>, span: TokenSpan) -> (Box<Scope>, Type) {
    let self_type = scope.get_self_type();
    if let Some(self_type) = self_type {
        (scope, self_type)
    } else {
        scope.source.print_error(
            span,
            "Invalid `self` outside of struct or enum",
            "`self` value only available inside of struct or enum",
        );
        (scope, Type::Error)
    }
}
