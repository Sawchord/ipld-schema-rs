use super::{parse_type_name, IpldSchemaParseError};
use crate::{
    bytes::{parse_bytes_representation, BytesRepresentation},
    IpldType,
};
use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::{map, opt},
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

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

pub(crate) fn parse_link(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("&"), space0, parse_type_name)),
        |(_, _, link)| IpldType::Link(link.to_string()),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse::parse_type_declaration, Doc};

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
}
