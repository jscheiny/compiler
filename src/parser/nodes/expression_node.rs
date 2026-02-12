use std::fmt::Display;

use crate::{
    checker::{Scope, ScopeType, Type, TypeResolver},
    parser::{
        BinaryOpExpressionNode, BlockNode, FunctionCallExpressionNode, IfExpressionNode,
        PostfixOpExpressionNode, PrefixOpExpressionNode, PrimitiveType,
    },
};

pub enum ExpressionNode {
    PrefixOp(PrefixOpExpressionNode),
    BinaryOp(BinaryOpExpressionNode),
    PostfixOp(PostfixOpExpressionNode),
    FunctionCall(FunctionCallExpressionNode),
    IfExpression(IfExpressionNode),
    BooleanLiteral(bool),
    IntegerLiteral(i64),
    StringLiteral(String),
    Block(BlockNode),
    Identifier(String),
    SelfRef(String),
    Error,
}

impl ExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        match self {
            Self::PrefixOp(node) => node.check(types, scope),
            Self::BinaryOp(node) => node.check(types, scope),
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

impl Display for ExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExpressionNode::PrefixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionNode::BinaryOp(node) => {
                write!(
                    f,
                    "{:?}({}, {})",
                    node.operator.value, node.left.value, node.right.value
                )
            }
            ExpressionNode::PostfixOp(node) => {
                write!(f, "{:?}({})", node.operator.value, node.expression.value)
            }
            ExpressionNode::FunctionCall(node) => {
                write!(f, "Call({}, (", node.function.value)?;
                for (index, arg) in node.arguments.iter().enumerate() {
                    write!(f, "{}", arg.value)?;
                    if index != node.arguments.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "))")
            }
            ExpressionNode::IfExpression(node) => {
                write!(
                    f,
                    "If({})Then({})Else({})",
                    node.predicate.value, node.if_true.value, node.if_false.value
                )
            }
            ExpressionNode::BooleanLiteral(literal) => write!(f, "{}", literal),
            ExpressionNode::StringLiteral(literal) => write!(f, "{}", literal),
            ExpressionNode::IntegerLiteral(literal) => write!(f, "{}", literal),
            ExpressionNode::Block(_) => write!(f, "[BLOCK]"),
            ExpressionNode::Identifier(identifier) => write!(f, "{}", identifier),
            ExpressionNode::SelfRef(identifier) => write!(f, "@{}", identifier),
            ExpressionNode::Error => write!(f, "[ERROR]"),
        }
    }
}
