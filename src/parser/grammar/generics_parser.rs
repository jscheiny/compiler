use crate::{
    lexer::Symbol,
    parser::{
        NameType, ParseResult, TokenStream, TypeParameterListNode, TypeParameterNode,
        grammar::comma_separated_list,
    },
};

pub fn type_parameter_list(tokens: &mut TokenStream) -> ParseResult<TypeParameterListNode> {
    let list = comma_separated_list(tokens, Symbol::CloseBracket, type_parameter)?;
    Ok(TypeParameterListNode::new(list))
}

fn type_parameter(tokens: &mut TokenStream) -> ParseResult<TypeParameterNode> {
    let name = tokens.name(NameType::Interface)?;
    Ok(TypeParameterNode { name })
}

// fn type_bounds(tokens: &mut TokenStream) -> ParseResult<Vec<Node<UserDefinedTypeNode>>> {
//     let mut bounds = vec![];
//     while !Symbol::CloseBracket.matches(tokens.peek()) && !Symbol::Comma.matches(tokens.peek()) {
//         bounds.push(tokens.located(user_defined_type)?);
//         if tokens.accept(Symbol::Plus) {
//             continue;
//         }
//     }

//     Ok(bounds)
// }
