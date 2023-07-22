use thiserror::Error;

use crate::Doc;

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
                doc: Some("Enum using string representation\n".to_string()),
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
                            doc: Some("This variant is selfdescribing\n".to_string()),
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
                doc: Some("Enum using integer representation\n".to_string()),
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
