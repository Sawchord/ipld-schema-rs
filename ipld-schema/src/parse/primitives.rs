use crate::{
    representation::{BytesRepresentation, UnitRepresentation},
    IpldType,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1},
    combinator::{map, opt},
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

use super::{parse_type_name, IpldSchemaParseError};

pub(crate) fn parse_bool(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(tag("bool"), |_| IpldType::Bool)(input)
}

pub(crate) fn parse_string(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(tag("string"), |_| IpldType::String)(input)
}

pub(crate) fn parse_int(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(tag("int"), |_| IpldType::Int)(input)
}

pub(crate) fn parse_float(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(tag("float"), |_| IpldType::Float)(input)
}

pub(crate) fn parse_any(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(tag("any"), |_| IpldType::Any)(input)
}

pub(crate) fn parse_bytes(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("bytes"), opt(parse_bytes_representation))),
        |(_, repr)| IpldType::Bytes(repr.unwrap_or(BytesRepresentation::Bytes)),
    )(input)
}

fn parse_bytes_representation(
    input: InStr,
) -> ParseResult<BytesRepresentation, IpldSchemaParseError> {
    map(
        tuple((
            space1,
            tag("representation"),
            space1,
            alt((
                map(parse_advanced, |advanced| {
                    BytesRepresentation::Advanced(advanced.to_string())
                }),
                map(tag("bytes"), |_| BytesRepresentation::Bytes),
            )),
        )),
        |(_, _, _, repr)| repr,
    )(input)
}

pub(crate) fn parse_link(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("&"), space0, parse_type_name)),
        |(_, _, link)| IpldType::Link(link.to_string()),
    )(input)
}

pub(crate) fn parse_unit(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("unit"), parse_unit_representation)),
        |(_, repr)| IpldType::Unit(repr),
    )(input)
}

fn parse_unit_representation(
    input: InStr,
) -> ParseResult<UnitRepresentation, IpldSchemaParseError> {
    map(
        tuple((
            space1,
            tag("representation"),
            space1,
            alt((
                map(tag("null"), |_| UnitRepresentation::Null),
                map(tag("true"), |_| UnitRepresentation::True),
                map(tag("false"), |_| UnitRepresentation::False),
                map(tag("emptymap"), |_| UnitRepresentation::EmptyMap),
            )),
        )),
        |(_, _, _, repr)| repr,
    )(input)
}

fn parse_advanced(input: InStr) -> ParseResult<InStr, IpldSchemaParseError> {
    map(
        tuple((tag("advanced"), space1, parse_type_name)),
        |(_, _, name)| name,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse::parse_type_declaration, Doc};

    use super::*;

    #[test]
    fn test_bool_declaration_uncommented() {
        let uncommented_bool = "type UncommentedBool bool";
        let expected_result = (
            "UncommentedBool".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Bool,
            },
        );

        assert_eq!(
            parse_type_declaration(uncommented_bool.into()).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_any_declaration_commented() {
        let commented_any = "\
            # This is the documentation of this type\n\
            #  \n\n\
            type Commented_Any any  \n";

        let expected_doc = "\
            This is the documentation of this type\n\
            \n\
        ";
        let expected_result = (
            "Commented_Any".to_string(),
            Doc {
                doc: Some(String::from(expected_doc)),
                ty: IpldType::Any,
            },
        );

        assert_eq!(
            parse_type_declaration(commented_any.into()).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_bytes_with_advances_repr() {
        let advanced_bytes = "\
        # These bytes are more advanced than normal bytes\n\
        type AdvancedBytes bytes representation advanced Taste\
        ";

        let expected_doc = "These bytes are more advanced than normal bytes\n";
        let expected_result = (
            "AdvancedBytes".to_string(),
            Doc {
                doc: Some(String::from(expected_doc)),
                ty: IpldType::Bytes(BytesRepresentation::Advanced("Taste".to_string())),
            },
        );

        assert_eq!(
            parse_type_declaration(advanced_bytes.into()).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_link() {
        let link = "type LinkedData &Data";
        let expected_result = (
            "LinkedData".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Link("Data".to_string()),
            },
        );
        assert_eq!(
            parse_type_declaration(link.into()).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_unit() {
        let unit = "type MyUnit unit representation true";
        let expexted_result = (
            "MyUnit".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Unit(UnitRepresentation::True),
            },
        );
        assert_eq!(
            parse_type_declaration(unit.into()).unwrap().1,
            expexted_result
        );
    }
}
