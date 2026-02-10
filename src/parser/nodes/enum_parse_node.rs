use std::collections::HashMap;

use crate::{
    checker::{DuplicateMemberName, EnumMember, EnumType, Type, TypeError, TypeResolver},
    parser::{
        EnumVariantParseNode, IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec,
        TokenSpan, Traverse,
    },
};

pub struct EnumParseNode {
    pub identifier: ParseNode<IdentifierParseNode>,
    pub variants: ParseNodeVec<EnumVariantParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for EnumParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Enum.identifier", self.identifier.span);
        visit("Enum.variants", self.variants.span);
        for variant in self.variants.value.iter() {
            variant.traverse("Enum.variant", visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit("Enum.methods", methods.span);
            for method in methods.value.iter() {
                method.traverse("Enum.method", visit);
            }
        }
    }
}

impl EnumParseNode {
    pub fn register_type(&self, types: &mut TypeResolver) {
        let mut members = HashMap::new();

        for variant in self.variants.value.iter() {
            let EnumVariantParseNode {
                identifier,
                type_def,
            } = &variant.value;
            let type_def = type_def.as_ref().map(|t| t.value.resolve_type(types));

            let identifier = &identifier.value.0;
            if members.contains_key(identifier) {
                types.push_error(self.create_duplicate_member_error(identifier));
                continue;
            }

            members.insert(identifier.clone(), EnumMember::Variant(type_def));
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.value.iter() {
                let MethodParseNode { function, .. } = &method.value;
                let function_type = function.value.resolve_type(types);

                let identifier = &function.value.identifier.value.0;
                if members.contains_key(identifier) {
                    types.push_error(self.create_duplicate_member_error(identifier));
                    continue;
                }

                members.insert(identifier.clone(), EnumMember::Method(function_type));
            }
        }

        types.insert(&self.identifier.value.0, Type::Enum(EnumType { members }))
    }

    fn create_duplicate_member_error(&self, member_name: &String) -> TypeError {
        TypeError::DuplicateMemberName(DuplicateMemberName {
            member_name: member_name.clone(),
            container_name: self.identifier.value.0.clone(),
            container_type: String::from("enum"),
        })
    }
}
