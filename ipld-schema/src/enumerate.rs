use crate::{comment::parse_comment, parse::IpldSchemaParseError, Rule};
use pest::iterators::Pairs;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InvalidEnum {
    #[error("Enum representation must either be \"int\" or \"string\", found \"{0}\"")]
    InvalidRepresentation(String),
    #[error("Enum member tag does not match representation")]
    InvalidMemberTag,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct EnumType {
    members: Vec<EnumField>,
    repr: EnumRepresentation,
}

pub(crate) fn parse_enum(enu: Pairs<Rule>) -> Result<EnumType, IpldSchemaParseError> {
    let mut members = vec![];
    let mut repr = None;

    for pair in enu {
        match pair.as_rule() {
            Rule::enum_field => members.push(parse_enum_field(pair.into_inner())?),
            Rule::enum_repr => {
                assert!(repr.is_none());
                repr = Some(parse_enum_representation(pair.into_inner()));
            }
            _ => panic!("Expected enum_field or enum_repr"),
        }
    }

    Ok(EnumType {
        members,
        repr: repr.unwrap_or(EnumRepresentation::String),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnumField {
    doc: Option<String>,
    name: String,
    tag: EnumMemberTag,
}

fn parse_enum_field(mut field: Pairs<Rule>) -> Result<EnumField, IpldSchemaParseError> {
    let doc = if field.peek().unwrap().as_rule() == Rule::comment {
        Some(parse_comment(field.next().unwrap().into_inner()))
    } else {
        None
    };

    let name = field.next().unwrap();
    let name = name.as_str().to_string();

    let tag = if let Some(repr) = field.next() {
        assert_eq!(repr.as_rule(), Rule::enum_field_repr);

        let mut inner = repr.into_inner();
        let val = inner.next().unwrap();
        assert!(inner.next().is_none());

        assert_eq!(val.as_rule(), Rule::enum_field_repr_value);
        match val.as_str().parse::<i128>() {
            Ok(int_val) => EnumMemberTag::Int(int_val),
            Err(_) => EnumMemberTag::String(val.as_str().to_string()),
        }
    } else {
        EnumMemberTag::String(name.clone())
    };

    Ok(EnumField { doc, name, tag })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EnumMemberTag {
    Int(i128),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumRepresentation {
    String,
    Int,
}

fn parse_enum_representation(mut repr: Pairs<Rule>) -> EnumRepresentation {
    let inner = repr.next().unwrap();
    assert!(repr.next().is_none());
    assert_eq!(inner.as_rule(), Rule::enum_repr_value);

    match inner.as_str() {
        "int" => EnumRepresentation::Int,
        "string" => EnumRepresentation::String,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Doc, IpldSchema, IpldType};
    use std::collections::BTreeMap;

    #[test]
    fn enumerate() {
        let file = include_str!("../test/enums.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());
        expected_schema.0.insert(
            "StatusString".to_string(),
            Doc {
                doc: Some("Enum using string representation".to_string()),
                ty: IpldType::Enum(EnumType {
                    members: vec![
                        EnumField {
                            doc: None,
                            name: "Nope".to_string(),
                            tag: EnumMemberTag::String("Nay".to_string()),
                        },
                        EnumField {
                            doc: None,
                            name: "Yep".to_string(),
                            tag: EnumMemberTag::String("Yay".to_string()),
                        },
                        EnumField {
                            doc: Some("This variant is selfdescribing".to_string()),
                            name: "Maybe".to_string(),
                            tag: EnumMemberTag::String("Maybe".to_string()),
                        },
                    ],
                    repr: EnumRepresentation::String,
                }),
            },
        );
        expected_schema.0.insert(
            "StatusInt".to_string(),
            Doc {
                doc: Some("Enum using integer representation".to_string()),
                ty: IpldType::Enum(EnumType {
                    members: vec![
                        EnumField {
                            doc: None,
                            name: "Nope".to_string(),
                            tag: EnumMemberTag::Int(0),
                        },
                        EnumField {
                            doc: None,
                            name: "Yep".to_string(),
                            tag: EnumMemberTag::Int(1),
                        },
                        EnumField {
                            doc: None,
                            name: "Maybe".to_string(),
                            tag: EnumMemberTag::Int(100),
                        },
                    ],
                    repr: EnumRepresentation::Int,
                }),
            },
        );

        assert_eq!(parsed_schema, expected_schema);
    }

    // TODO:  Parse a file with mismatching representation tags
}
