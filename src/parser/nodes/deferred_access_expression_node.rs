use crate::{
    checker::{FunctionType, Scope, Type},
    parser::{
        ExpressionNode, IdentifierNode, Node, NodeVec, TokenSpan, check_function_call, get_field,
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
                scope.source.print_type_error(
                    // TODO this span should cover the whole node...
                    self.field.span,
                    "Deferred access expression must be a single parameter function",
                    &format!(
                        "expected type takes {} parameters",
                        function_type.parameters.len(),
                    ),
                );
            }

            let parameter_type = function_type.parameters.swap_remove(0);
            let field_type = get_field(&parameter_type, &self.field, &scope).unwrap_or(Type::Error);
            let (scope, result_type) = if let Some(arguments) = self.arguments.as_ref() {
                let field_function_type = field_type.as_function(&scope.types);
                check_function_call(scope, field_function_type.as_ref(), &arguments.value)
            } else {
                (scope, field_type)
            };

            let function_type = FunctionType::new(parameter_type, result_type);
            (scope, Type::Function(function_type))
        } else {
            let span = TokenSpan::singleton_of(self.field.span.start_index - 1);
            scope.source.print_type_error(
                span,
                "Deferred access type is ambiguous",
                "could not infer type of implicit parameter",
            );
            (scope, Type::Error)
        }
    }
}
