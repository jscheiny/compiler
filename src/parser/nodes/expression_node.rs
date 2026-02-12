use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{
        AccessExpressionNode, BinaryOpExpressionNode, BlockNode, FunctionCallExpressionNode,
        IfExpressionNode, PostfixOpExpressionNode, PrefixOpExpressionNode, PrimitiveType,
    },
};

pub enum ExpressionNode {
    PrefixOp(PrefixOpExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    Access(AccessExpressionNode),
    PostfixOp(PostfixOpExpressionNode),
    SelfRef(String),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    BooleanLiteral(bool),
    IntegerLiteral(i64),
    StringLiteral(String),
    Block(BlockNode),
    Identifier(String),
    Error,
}

impl ExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match self {
            Self::PrefixOp(node) => node.check(types, scope),
            Self::BinaryOp(node) => node.check(types, scope),
            Self::Access(_) => todo!("Implement type checking for ExpressionNode::Access"),
            Self::PostfixOp(node) => node.check(types, scope),
            Self::FunctionCall(node) => {
                let (scope, resolved_type) = node.check(types, scope);
                (scope, resolved_type.unwrap_or(Type::Error))
            }
            Self::IfExpression(node) => node.check(types, scope),
            Self::BooleanLiteral(_) => (scope, Type::Primitive(PrimitiveType::Bool)),
            Self::IntegerLiteral(_) => (scope, Type::Primitive(PrimitiveType::Int)),
            Self::StringLiteral(_) => {
                todo!("Implement type checking for ExpressionNode::StringLiteral")
            }
            Self::Block(node) => {
                // TODO handle type checking of block results
                let (scope, resolved_type) = node.check(types, scope);
                (scope, resolved_type.unwrap_or(Type::Error))
            }
            Self::Identifier(identifier) => {
                let resolved_type = scope.lookup(identifier);
                if resolved_type.is_none() {
                    println!("Type Error: Could not find symbol `{}`", identifier)
                }
                (scope, resolved_type.unwrap_or(Type::Error))
            }
            Self::SelfRef(identifier) => self.check_self_ref(identifier, scope),
            Self::Error => (scope, Type::Error),
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
