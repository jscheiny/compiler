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
        let (scope, function_type) = self.function.check(types, scope, None);
        let function_type = get_function_type(&function_type, types);
        // TODO combine get args and check args so that we can pass in expected types
        let (scope, arguments) = self.get_args(types, scope);

        if let Some(function_type) = function_type {
            self.check_args(&function_type, &arguments, types);
            (scope, *function_type.return_type)
        } else {
            (scope, Type::Error)
        }
    }

    fn get_args(&self, types: &TypeResolver, mut scope: Box<Scope>) -> (Box<Scope>, Vec<Type>) {
        let mut result = vec![];
        for argument in self.arguments.iter() {
            // TODO pass in the parameter type here...
            let (new_scope, resolved_type) = argument.check(types, scope, None);
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

        for (index, (left, right)) in function_type.parameters.iter().zip(arguments).enumerate() {
            if !left.is_assignable_to(right, types) {
                println!("Type error: Mismatch in type of argument {}", index);
            }
        }
        // TODO check types of each argument
    }
}

// TODO move this elsewhere
pub fn get_function_type(input_type: &Type, types: &TypeResolver) -> Option<FunctionType> {
    match input_type {
        Type::Function(function_type) => Some(function_type.clone()),
        Type::Reference(index) => {
            let resolved_type = types.get_type(*index).unwrap();
            get_function_type(&resolved_type, types)
        }
        Type::Type(_) => todo!("Implement call operator for types"),
        Type::Enum(_)
        | Type::Primitive(_)
        | Type::Struct(_)
        | Type::Tuple(_)
        | Type::Void
        | Type::Error => None,
    }
}
