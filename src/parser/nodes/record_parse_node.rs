use std::collections::HashMap;

use crate::{
    checker::{
        DuplicateMemberName, ResolveType, StructDeclaration, StructDeclarationType, StructType,
        Type, TypeError, TypeResolver,
    },
    parser::{
        IdentifierParseNode, MethodParseNode, ParseNode, ParseNodeVec, RecordFieldParseNode,
        TokenSpan, Traverse,
    },
};

pub enum RecordType {
    Struct,
    Tuple,
}

pub struct RecordDefinitionParseNode {
    pub record_type: RecordType,
    pub identifier: ParseNode<IdentifierParseNode>,
    pub fields: ParseNodeVec<RecordFieldParseNode>,
    pub methods: Option<ParseNodeVec<MethodParseNode>>,
}

impl Traverse for RecordDefinitionParseNode {
    fn traverse(&self, visit: &impl Fn(&str, TokenSpan)) {
        visit("Record.identifier", self.identifier.span);
        visit("Record.fields", self.fields.span);
        for field in self.fields.value.iter() {
            field.traverse("Record.field", visit);
        }
        if let Some(methods) = self.methods.as_ref() {
            visit("Record.methods", methods.span);
            for method in methods.value.iter() {
                method.traverse("Record.method", visit);
            }
        }
    }
}

impl RecordDefinitionParseNode {
    pub fn register_type(&self, types: &mut TypeResolver) {
        let mut result = StructType {
            declarations: HashMap::new(),
        };

        for field in self.fields.value.iter() {
            let RecordFieldParseNode {
                identifier,
                type_def,
                public,
            } = &field.value;
            let declaration_type = match type_def {
                Some(type_def) => type_def.value.resolve_type(types),
                None => Type::Error,
            };

            let identifier = &identifier.value.0;
            if result.declarations.contains_key(identifier) {
                types.push_error(self.create_duplicate_member_error(identifier));
                continue;
            }

            result.declarations.insert(
                identifier.clone(),
                StructDeclaration {
                    public: *public,
                    declaration_type: StructDeclarationType::Field(declaration_type),
                },
            );
        }

        if let Some(methods) = self.methods.as_ref() {
            for method in methods.value.iter() {
                let MethodParseNode { public, function } = &method.value;
                let function_type = function.value.resolve_type(types);

                let identifier = &function.value.identifier.value.0;
                if result.declarations.contains_key(identifier) {
                    types.push_error(self.create_duplicate_member_error(identifier));
                    continue;
                }

                result.declarations.insert(
                    identifier.clone(),
                    StructDeclaration {
                        public: *public,
                        declaration_type: StructDeclarationType::Method(function_type),
                    },
                );
            }
        }

        types.insert(&self.identifier.value.0, Type::Struct(result))
    }

    fn create_duplicate_member_error(&self, member_name: &String) -> TypeError {
        TypeError::DuplicateMemberName(DuplicateMemberName {
            member_name: member_name.clone(),
            container_name: self.identifier.value.0.clone(),
            container_type: match self.record_type {
                RecordType::Struct => String::from("struct"),
                RecordType::Tuple => String::from("tuple"),
            },
        })
    }
}
