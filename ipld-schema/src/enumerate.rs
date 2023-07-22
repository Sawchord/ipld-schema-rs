use crate::{comment::parse_comment_block, parse::IpldSchemaParseError, Doc, IpldType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::{multispace0, multispace1, space0, space1},
    combinator::{map, opt, peek},
    multi::many1,
    sequence::tuple,
    AsChar,
};
use nom_diagnostic::{diagnose, map_diagnose, span, ErrorDiagnose, InStr, ParseResult, Span};
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

pub(crate) fn parse_enum(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map_diagnose(
        tuple((
            tag("enum"),
            space0,
            tag("{"),
            parse_enum_members,
            tag("}"),
            opt(parse_enum_representation),
        )),
        |(_, _, _, members, _, representation)| {
            // If no representation is given, we default to string
            let representation = representation
                .map(|x| x.into_inner())
                .unwrap_or(EnumRepresentationTag::String);

            // Parse the representation tags
            // This fails, if the tags are inconsistent with the representation specification
            // If the representation tag is a string, check that all member tags are strings
            let representation = match representation {
                EnumRepresentationTag::String => {
                    let tags = members
                        .clone()
                        .into_iter()
                        .map(|enum_member| {
                            enum_member
                                .map(|(_, _, tag)| match tag {
                                    EnumMemberTag::Int(_) => Err(IpldSchemaParseError::Enum(
                                        InvalidEnum::InvalidMemberTag,
                                    )),
                                    EnumMemberTag::String(name) => Ok(name),
                                })
                                .with_hint(
                                    "enum member tag is an integer but representation is string",
                                )
                                .transform()
                        })
                        .collect::<Result<_, _>>()
                        .map_err(ErrorDiagnose::from)?;
                    EnumRepresentation::String(tags)
                }

                // If the representation tag is an int, we need to check that all values are ints
                EnumRepresentationTag::Int => {
                    let tags = members
                        .clone()
                        .into_iter()
                        .map(|enum_member| {
                            enum_member
                                .map(|(_, _, tag)| match tag {
                                    EnumMemberTag::Int(int) => Ok(int),
                                    EnumMemberTag::String(_) => Err(IpldSchemaParseError::Enum(
                                        InvalidEnum::InvalidMemberTag,
                                    )),
                                })
                                .with_hint(
                                    "enum member tag is a string but representation is an integer",
                                )
                                .transform()
                        })
                        .collect::<Result<_, _>>()
                        .map_err(ErrorDiagnose::from)?;
                    EnumRepresentation::Int(tags)
                }
            };

            let members = members
                .into_iter()
                .map(|enum_member| enum_member.into_inner())
                .map(|(comment, name, _)| Doc {
                    doc: comment,
                    ty: name,
                })
                .collect();

            Ok::<_, ErrorDiagnose<'_, _>>(IpldType::Enum(EnumType {
                members,
                representation,
            }))
        },
    )(input)
}

#[allow(clippy::type_complexity)]
fn parse_enum_members(
    input: InStr,
) -> ParseResult<Vec<Span<(Option<String>, String, EnumMemberTag)>>, IpldSchemaParseError> {
    many1(parse_enum_member)(input)
}

fn parse_enum_member(
    input: InStr,
) -> ParseResult<Span<(Option<String>, String, EnumMemberTag)>, IpldSchemaParseError> {
    span(map(
        tuple((
            opt(parse_comment_block),
            multispace0,
            tag("|"),
            space1,
            parse_enum_member_name,
            opt(parse_enum_member_tag),
            multispace1,
        )),
        |(comment, _, _, _, name, tag, _)| {
            (
                comment,
                name.to_string(),
                tag.unwrap_or(EnumMemberTag::String(name.to_string())),
            )
        },
    ))(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum EnumMemberTag {
    Int(i128),
    String(String),
}

fn parse_enum_member_tag(input: InStr) -> ParseResult<EnumMemberTag, IpldSchemaParseError> {
    map(
        tuple((
            space1,
            tag("("),
            space0,
            tag("\""),
            space0,
            alt((
                map(parse_enum_member_name, |name| {
                    EnumMemberTag::String(name.to_string())
                }),
                map(nom::character::complete::i128, |int| {
                    EnumMemberTag::Int(int)
                }),
            )),
            space0,
            tag("\""),
            space0,
            tag(")"),
        )),
        |tag| tag.5,
    )(input)
}

fn parse_enum_member_name(input: InStr) -> ParseResult<InStr, IpldSchemaParseError> {
    map(
        tuple((
            peek(take_while1(|c: char| c.is_alpha())),
            take_till1(|c: char| !(c.is_alphanum() || c == '_')),
        )),
        |(_, x)| x,
    )(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EnumRepresentation {
    String(Vec<String>),
    Int(Vec<i128>),
}

fn parse_enum_representation(
    input: InStr,
) -> ParseResult<Span<EnumRepresentationTag>, IpldSchemaParseError> {
    span(map(
        tuple((
            space1,
            tag("representation"),
            space1,
            diagnose(
                alt((
                    map(tag("int"), |_| EnumRepresentationTag::Int),
                    map(tag("string"), |_| EnumRepresentationTag::String),
                )),
                |error: nom::error::Error<_>| {
                    error
                        .input
                        .error_span(
                            |c| !c.is_alphanumeric(),
                            |name| {
                                IpldSchemaParseError::Enum(InvalidEnum::InvalidRepresentation(
                                    name.to_string(),
                                ))
                            },
                        )
                        .with_hint("this is not a valid value")
                },
            ),
        )),
        |(_, _, _, tag)| tag,
    ))(input)
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
