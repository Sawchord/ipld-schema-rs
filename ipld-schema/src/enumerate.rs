use crate::{comment::parse_comment, parse::IpldSchemaParseError, Doc, IpldType, Rule};
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
    members: Vec<Doc<String>>,
    representation: EnumRepresentation,
}

pub(crate) fn parse_enum(enu: Pairs<Rule>) -> Result<IpldType, IpldSchemaParseError> {
    let mut fields = vec![];
    let mut repr = None;

    for pair in enu {
        match pair.as_rule() {
            Rule::enum_field => fields.push(parse_enum_field(pair.into_inner())?),
            Rule::enum_repr => {
                assert!(repr.is_none());
                let mut outer = pair.into_inner();

                let inner = outer.next().unwrap();
                assert!(outer.next().is_none());
                assert_eq!(inner.as_rule(), Rule::enum_repr_value);

                repr = match inner.as_str() {
                    "int" => Some(EnumRepresentationTag::Int),
                    "string" => Some(EnumRepresentationTag::String),
                    repr => {
                        return Err(IpldSchemaParseError::Enum(
                            InvalidEnum::InvalidRepresentation(repr.to_string()),
                        ))
                    }
                };
            }
            _ => panic!("Expected enum_field or enum_repr"),
        }
    }

    let mut members = vec![];
    let representation = match repr.unwrap_or(EnumRepresentationTag::String) {
        EnumRepresentationTag::String => {
            let mut representation = vec![];
            for (comment, name, repr) in fields {
                members.push(Doc {
                    doc: comment,
                    ty: name,
                });
                let EnumMemberTag::String(val) = repr else {
                    return Err(IpldSchemaParseError::Enum(InvalidEnum::InvalidMemberTag))
                };
                representation.push(val);
            }
            EnumRepresentation::String(representation)
        }
        EnumRepresentationTag::Int => {
            let mut representation = vec![];
            for (comment, name, repr) in fields {
                members.push(Doc {
                    doc: comment,
                    ty: name,
                });
                let EnumMemberTag::Int(val) = repr else {
                    return Err(IpldSchemaParseError::Enum(InvalidEnum::InvalidMemberTag))
                };
                representation.push(val);
            }
            EnumRepresentation::Int(representation)
        }
    };

    Ok(IpldType::Enum(EnumType {
        members,
        representation,
    }))
}

fn parse_enum_field(
    mut field: Pairs<Rule>,
) -> Result<(Option<String>, String, EnumMemberTag), IpldSchemaParseError> {
    let comment = if field.peek().unwrap().as_rule() == Rule::comment {
        Some(parse_comment(field.next().unwrap().into_inner()))
    } else {
        None
    };

    let name = field.next().unwrap();
    let name = name.as_str().to_string();

    let repr = if let Some(repr) = field.next() {
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

    Ok((comment, name, repr))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EnumMemberTag {
    Int(i128),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumRepresentation {
    String(Vec<String>),
    Int(Vec<i128>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumRepresentationTag {
    Int,
    String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Doc, IpldSchema, IpldType};
    use std::collections::BTreeMap;

    #[test]
    fn test_enum_parse() {
        let file = include_str!("../test/enums.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());
        expected_schema.0.insert(
            "StatusString".to_string(),
            Doc {
                doc: Some("Enum using string representation".to_string()),
                ty: IpldType::Enum(EnumType {
                    members: vec![
                        Doc {
                            doc: None,
                            ty: "Nope".to_string(),
                        },
                        Doc {
                            doc: None,
                            ty: "Yep".to_string(),
                        },
                        Doc {
                            doc: Some("This variant is selfdescribing".to_string()),
                            ty: "Maybe".to_string(),
                        },
                    ],
                    representation: EnumRepresentation::String(vec![
                        "Nay".to_string(),
                        "Yay".to_string(),
                        "Maybe".to_string(),
                    ]),
                }),
            },
        );
        expected_schema.0.insert(
            "StatusInt".to_string(),
            Doc {
                doc: Some("Enum using integer representation".to_string()),
                ty: IpldType::Enum(EnumType {
                    members: vec![
                        Doc {
                            doc: None,
                            ty: "Nope".to_string(),
                        },
                        Doc {
                            doc: None,
                            ty: "Yep".to_string(),
                        },
                        Doc {
                            doc: None,
                            ty: "Maybe".to_string(),
                        },
                    ],
                    representation: EnumRepresentation::Int(vec![0, 1, 100]),
                }),
            },
        );

        assert_eq!(parsed_schema, expected_schema);
    }

    // TODO:  Parse a file with mismatching representation tags
}
