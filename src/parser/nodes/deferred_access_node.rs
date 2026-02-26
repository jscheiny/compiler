use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{ExpressionNode, Identified, IdentifierNode, Node, NodeVec, get_field},
};

pub struct DeferredAccessNode {
    pub field: Node<IdentifierNode>,
    pub arguments: Option<NodeVec<ExpressionNode>>,
}

impl DeferredAccessNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let function_type = expected_type.and_then(|t| t.clone().as_function(&scope.types));
        if let Some(mut function_type) = function_type {
            if function_type.parameters.len() != 1 {
                println!(
                    "Type error: Expected deferred access to be assigned to function taking exactly one parameter"
                );
            }

            let parameter_type = function_type.parameters.swap_remove(0);
            let field_type =
                get_field(&parameter_type, self.field.id(), &scope.types).unwrap_or(Type::Error);
            let function_type = FunctionType::new(parameter_type, field_type);
            (scope, Type::Function(function_type))
        } else {
            println!("Type error: Could not infer type of deferred access in non-function context");
            (scope, Type::Error)
        }
    }
}
