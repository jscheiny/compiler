use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{
        AccessExpressionNode, ArrayExpressionNode, BinaryOpExpressionNode, BlockNode,
        ClosureExpressionNode, ClosureParameterExpressionNode, FunctionCallExpressionNode,
        IfExpressionNode, PostfixOpExpressionNode, PrefixOpExpressionNode, PrimitiveType,
    },
};

pub enum ExpressionNode {
    PrefixOp(PrefixOpExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    Access(AccessExpressionNode),
    PostfixOp(PostfixOpExpressionNode),
    Closure(ClosureExpressionNode),
    ClosureParameter(ClosureParameterExpressionNode),
    SelfRef(String),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    Array(ArrayExpressionNode),
    BooleanLiteral(bool),
    CharacterLiteral(String),
    IntegerLiteral(i64),
    StringLiteral(String),
    Block(BlockNode),
    Identifier(String),
    Error,
}

impl ExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        self.check_expected(types, scope, None)
    }

    pub fn check_expected(
        &self,
        types: &TypeResolver,
        scope: Box<Scope>,
        expected_type: Option<&Type>,
    ) -> (Box<Scope>, Type) {
        match self {
            Self::PrefixOp(node) => node.check(types, scope),
            Self::BinaryOp(node) => node.check(types, scope),
            Self::Access(node) => node.check(types, scope),
            Self::PostfixOp(node) => node.check(types, scope),
            Self::Closure(node) => node.check(types, scope, expected_type),
            Self::FunctionCall(node) => node.check(types, scope),
            Self::IfExpression(node) => node.check(types, scope, expected_type),
            Self::Array(node) => node.check(types, scope, expected_type),
            Self::BooleanLiteral(_) => (scope, Type::Primitive(PrimitiveType::Bool)),
            Self::CharacterLiteral(_) => (scope, Type::Primitive(PrimitiveType::Char)),
            Self::IntegerLiteral(_) => (scope, Type::Primitive(PrimitiveType::Int)),
            Self::StringLiteral(_) => {
                todo!("Implement type checking for ExpressionNode::StringLiteral")
            }
            Self::Block(node) => {
                let (scope, resolved_type) = node.check(types, scope, expected_type);
                (scope, resolved_type.unwrap_or(Type::Void))
            }
            Self::Identifier(identifier) => self.check_identifier(identifier, types, scope),
            Self::SelfRef(identifier) => self.check_self_ref(identifier, scope),
            Self::Error => (scope, Type::Error),
            Self::ClosureParameter(_) => {
                panic!("ERROR: Unexpected closure parameter outside of parameter list")
            }
        }
    }

    fn check_identifier(
        &self,
        identifier: &String,
        types: &TypeResolver,
        scope: Box<Scope>,
    ) -> (Box<Scope>, Type) {
        // TODO disallow use of types as values
        if let Some(resolved_type) = scope.lookup(identifier) {
            (scope, resolved_type)
        } else if let Some(resolved_type) = types.get_ref(identifier).map(Type::Reference) {
            let resolved_type = resolved_type.as_runtime_type(types).map(Type::Type);
            match resolved_type {
                Some(resolved_type) => (scope, resolved_type),
                None => {
                    println!("Type Error: Could not resolve `{}` as a value", identifier);
                    (scope, Type::Error)
                }
            }
        } else {
            println!("Type Error: Could not find symbol `{}`", identifier);
            (scope, Type::Error)
        }
    }

    fn check_self_ref(&self, identifier: &String, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let self_scope = scope.find_scope(ScopeType::Struct);
        if let Some(self_scope) = self_scope {
            let resolved_type = self_scope.lookup_local(identifier);
            if let Some(resolved_type) = resolved_type {
                return (scope, resolved_type);
            }
            println!("Type error: cannot find value in struct or enum");
        } else {
            println!("Type error: Cannot use @ op outside of struct or enum");
        }

        (scope, Type::Error)
    }
}
