use crate::{
    checker::{FunctionType, Scope, Type, TypeResolver},
    parser::{ExpressionNode, Node, NodeVec},
};

pub struct FunctionCallExpressionNode {
    pub function: Box<Node<ExpressionNode>>,
    pub arguments: NodeVec<ExpressionNode>,
}

impl FunctionCallExpressionNode {
    pub fn check(&self, types: &TypeResolver, scope: Box<Scope>) -> (Box<Scope>, Type) {
        let (scope, function_type) = self.function.check(types, scope);
        let function_type = function_type.as_function(types);
        // TODO combine get args and check args so that we can pass in expected types
        let (scope, arguments) = self.get_args(types, scope, function_type.as_ref());

        if let Some(function_type) = function_type {
            self.check_args(&function_type, &arguments, types);
            (scope, *function_type.return_type)
        } else {
            (scope, Type::Error)
        }
    }

    fn get_args(
        &self,
        types: &TypeResolver,
        mut scope: Box<Scope>,
        function_type: Option<&FunctionType>,
    ) -> (Box<Scope>, Vec<Type>) {
        let mut result = vec![];
        for (index, argument) in self.arguments.iter().enumerate() {
            let parameter_type = function_type.and_then(|ft| ft.parameters.get(index));
            let (new_scope, resolved_type) = argument.check_expected(types, scope, parameter_type);
            result.push(resolved_type);
            scope = new_scope;
        }
        (scope, result)
    }

    fn check_args(&self, function_type: &FunctionType, arguments: &[Type], types: &TypeResolver) {
        if function_type.parameters.len() != arguments.len() {
            println!(
                "Type error: Call expected {} arguments but recieved {}",
                function_type.parameters.len(),
                arguments.len()
            );
        }

        for (index, (parameter, argument)) in
            function_type.parameters.iter().zip(arguments).enumerate()
        {
            if !argument.is_assignable_to(parameter, types) {
                println!(
                    "Type error: Arg {} of type `{}` cannot be assigned to parameter of type `{}`",
                    index,
                    argument.format(types),
                    parameter.format(types),
                );
            }
        }
        // TODO check types of each argument
    }
}
