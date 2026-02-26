use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{
        ExpressionNode, Identified, IdentifierNode, Node, NodeVec, check_function_call, get_field,
    },
};

pub struct DeferredAccessExpressionNode {
    pub field: Node<IdentifierNode>,
    pub arguments: Option<NodeVec<ExpressionNode>>,
}

impl DeferredAccessExpressionNode {
    pub fn check(&self, scope: Box<Scope>, expected_type: Option<&Type>) -> (Box<Scope>, Type) {
        let function_type = expected_type.and_then(|t| t.clone().as_function(&scope.types));
        if let Some(mut function_type) = function_type {
            if function_type.parameters.len() != 1 {
                println!(
                    "Type error: Expected deferred access expression to be assigned to function taking exactly one parameter"
                );
            }

            let parameter_type = function_type.parameters.swap_remove(0);
            let field_type =
                get_field(&parameter_type, self.field.id(), &scope.types).unwrap_or(Type::Error);
            let (scope, result_type) = if let Some(arguments) = self.arguments.as_ref() {
                let field_function_type = field_type.as_function(&scope.types);
                check_function_call(scope, field_function_type.as_ref(), &arguments.value)
            } else {
                (scope, field_type)
            };

            let function_type = FunctionType::new(parameter_type, result_type);
            (scope, Type::Function(function_type))
        } else {
            println!("Type error: Could not infer type of deferred access in non-function context");
            (scope, Type::Error)
        }
    }
}
