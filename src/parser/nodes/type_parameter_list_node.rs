use std::{
    cell::OnceCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    checker::{Scope, Type, TypeParameter, TypeParameterList, TypeParameterMap},
    parser::{Node, TokenSpan, TypeParameterNode},
};

pub struct TypeParameterListNode {
    pub list: Vec<Node<TypeParameterNode>>,
    types: OnceCell<TypeParameterListNodeData>,
}

struct TypeParameterListNodeData {
    types_list: TypeParameterList,
    types_map: TypeParameterMap,
}

impl TypeParameterListNode {
    pub fn new(list: Vec<Node<TypeParameterNode>>) -> Self {
        Self {
            list,
            types: OnceCell::new(),
        }
    }

    pub fn check(&self, mut scope: Box<Scope>, span: TokenSpan) -> Box<Scope> {
        if self.list.is_empty() {
            scope.source.print_error(
                span,
                "Type parameter list should not be empty",
                "must provide at least one type parameter",
            );
        }

        let mut names = HashSet::new();
        for type_param in self.list.iter() {
            if !names.insert(&type_param.name.value) {
                scope.source.print_error(
                    type_param.name.span,
                    &format!("Duplicate type parameter name `{}`", type_param.name),
                    "type alias already contains a type parameter with this name",
                );
            } else {
                let type_parameter = Rc::new(TypeParameter {
                    name: type_param.name.clone(),
                });
                scope.add_type(&type_param.name, Type::TypeParameter(type_parameter));
            }
        }

        scope
    }

    pub fn get_types_list(&self) -> &TypeParameterList {
        &self.get_data().types_list
    }

    pub fn get_types_map(&self) -> &TypeParameterMap {
        &self.get_data().types_map
    }

    fn get_data(&self) -> &TypeParameterListNodeData {
        self.types.get_or_init(|| self.init_types())
    }

    fn init_types(&self) -> TypeParameterListNodeData {
        let mut types_map = HashMap::new();
        let mut types_list = vec![];
        for node in self.list.iter() {
            let type_param = Rc::new(TypeParameter {
                name: node.name.clone(),
            });
            types_map
                .entry(node.name.value.clone())
                .or_insert(type_param.clone());
            types_list.push(type_param);
        }

        TypeParameterListNodeData {
            types_list: TypeParameterList::new(types_list),
            types_map,
        }
    }
}
