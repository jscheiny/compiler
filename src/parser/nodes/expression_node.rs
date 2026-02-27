use crate::{
    checker::{Scope, ScopeType, Type},
    parser::{
        AccessExpressionNode, ArrayExpressionNode, BinaryOpExpressionNode, BlockNode,
        ClosureExpressionNode, ClosureParameterExpressionNode, DeferredAccessExpressionNode,
        FunctionCallExpressionNode, Identified, IdentifierNode, IfExpressionNode, MatchNode, Node,
        PostfixOpExpressionNode, PrefixOpExpressionNode, PrimitiveType, TokenSpan,
    },
};

pub enum ExpressionNode {
    Access(AccessExpressionNode),
    Array(ArrayExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    Block(BlockNode),
    BooleanLiteral(bool),
    CharacterLiteral(String),
    Closure(ClosureExpressionNode),
    ClosureParameter(ClosureParameterExpressionNode),
    DeferredAccess(DeferredAccessExpressionNode),
    FunctionCall(FunctionCallExpressionNode),
    Identifier(Node<IdentifierNode>),
    IfExpression(IfExpressionNode),
    IntegerLiteral(i64),
    Match(MatchNode),
    PostfixOp(PostfixOpExpressionNode),
    PrefixOp(PrefixOpExpressionNode),
    SelfRef(Node<IdentifierNode>),
    SelfValue(TokenSpan),
    StringLiteral(String),
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
            Self::Access(node) => node.check(scope, expected_type),
            Self::Array(node) => node.check(scope, expected_type),
            Self::BinaryOp(node) => node.check(scope, expected_type),
            Self::Block(node) => {
                let (scope, resolved_type) = node.check(scope, expected_type);
                (scope, resolved_type.unwrap_or(Type::Void))
            }
            Self::BooleanLiteral(_) => (scope, Type::Primitive(PrimitiveType::Bool)),
            Self::CharacterLiteral(_) => (scope, Type::Primitive(PrimitiveType::Char)),
            Self::Closure(node) => node.check(scope, expected_type),
            Self::ClosureParameter(_) => {
                panic!("ERROR: Unexpected closure parameter outside of parameter list")
            }
            Self::DeferredAccess(node) => node.check(scope, expected_type),
            Self::FunctionCall(node) => node.check(scope, expected_type),
            Self::Identifier(identifier) => self.check_identifier(identifier, scope, expected_type),
            Self::IfExpression(node) => node.check(scope, expected_type),
            Self::IntegerLiteral(_) => (scope, Type::Primitive(PrimitiveType::Int)),
            Self::Match(node) => node.check(scope, expected_type),
            Self::PostfixOp(node) => node.check(scope),
            Self::PrefixOp(node) => node.check(scope),
            Self::SelfRef(identifier) => self.check_self_ref(identifier, scope),
            Self::SelfValue(span) => self.check_self_value(*span, scope),
            Self::StringLiteral(_) => (
                scope,
                Type::Array(Box::new(Type::Primitive(PrimitiveType::Char))),
            ),
            Self::Error => (scope, Type::Error),
        }
    }

    fn check_identifier(
        &self,
        identifier: &Node<IdentifierNode>,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        let expected_enum_type = expected_type.and_then(|e| match e.deref(&scope.types) {
            Type::Enum(enum_type) => Some(enum_type),
            _ => None,
        });

        // TODO disallow use of types as values
        if let Some(resolved_type) = scope.lookup(identifier.id()) {
            (scope, resolved_type)
        } else if let Some(resolved_type) =
            scope.types.get_ref(identifier.id()).map(Type::Reference)
        {
            let resolved_type = resolved_type.as_runtime_type(&scope.types).map(Type::Type);
            match resolved_type {
                Some(resolved_type) => (scope, resolved_type),
                None => {
                    scope.source.print_error(
                        identifier.span,
                        "Invalid type as value",
                        "cannot use type as a value",
                    );
                    (scope, Type::Error)
                }
            }
        } else if let Some(enum_type) = expected_enum_type {
            if let Some(variant_type) = enum_type.get_variant(identifier.id()) {
                (scope, variant_type)
            } else {
                scope.source.print_error(
                    identifier.span,
                    &format!("Could not find value `{}`", identifier.id()),
                    "no such symbol found",
                );
                (scope, Type::Error)
            }
        } else {
            scope.source.print_error(
                identifier.span,
                &format!("Could not find value `{}`", identifier.id()),
                "no such symbol found",
            );
            (scope, Type::Error)
        }
    }

    fn check_self_ref(
        &self,
        identifier: &Node<IdentifierNode>,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Type) {
        let self_scope = scope.find_scope(|scope_type| matches!(scope_type, ScopeType::Struct(_)));
        if let Some(self_scope) = self_scope {
            let resolved_type = self_scope.lookup_local(identifier.id());
            if let Some(resolved_type) = resolved_type {
                return (scope, resolved_type);
            }
            scope.source.print_error(
                identifier.span,
                &format!("Could not find member `{}`", identifier.id()),
                "self type does not contain a member with this name",
            );
        } else {
            scope.source.print_error(
                identifier.span.previous(),
                "Self reference outside of struct or enum",
                "operator invalid outside of struct or enum",
            );
        }

        (scope, Type::Error)
    }

    fn check_self_value(&self, span: TokenSpan, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let mut self_index = None;
        scope.find_scope(|scope_type| match scope_type {
            ScopeType::Struct(index) => {
                self_index = Some(index);
                true
            }
            _ => false,
        });

        if let Some(index) = self_index {
            (scope, Type::Reference(index))
        } else {
            scope.source.print_error(
                span,
                "Invalid `self` outside of struct or enum",
                "`self` value only available inside of struct or enum",
            );
            (scope, Type::Error)
        }
    }
}
