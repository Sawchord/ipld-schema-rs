use crate::{representation::BytesRepresentation, IpldType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    sequence::tuple,
    IResult,
};

use super::parse_type_name;

pub(crate) fn parse_bool(input: &str) -> IResult<&str, IpldType> {
    map(tag("bool"), |_| IpldType::Bool)(input)
}

pub(crate) fn parse_string(input: &str) -> IResult<&str, IpldType> {
    map(tag("string"), |_| IpldType::String)(input)
}

pub(crate) fn parse_int(input: &str) -> IResult<&str, IpldType> {
    map(tag("int"), |_| IpldType::Int)(input)
}

pub(crate) fn parse_float(input: &str) -> IResult<&str, IpldType> {
    map(tag("float"), |_| IpldType::Float)(input)
}

pub(crate) fn parse_any(input: &str) -> IResult<&str, IpldType> {
    map(tag("any"), |_| IpldType::Any)(input)
}

pub(crate) fn parse_bytes(input: &str) -> IResult<&str, IpldType> {
    map(
        tuple((tag("bytes"), opt(parse_bytes_representation))),
        |(_, repr)| IpldType::Bytes(repr.unwrap_or(BytesRepresentation::Bytes)),
    )(input)
}

fn parse_bytes_representation(input: &str) -> IResult<&str, BytesRepresentation> {
    map(
        tuple((
            multispace0,
            tag("representation"),
            multispace0,
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

// TODO: Link
// TODO: Unit

fn parse_advanced(input: &str) -> IResult<&str, &str> {
    map(
        tuple((tag("advanced"), multispace0, parse_type_name)),
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
            parse_type_declaration(uncommented_bool).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_any_declaration_commented() {
        let commented_any = "\
            ## This is the documentation of this type\n\
            ##  \n\n\
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
            parse_type_declaration(commented_any).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_bytes_with_advances_repr() {
        let advanced_bytes = "\
        ## These bytes are more advanced than normal bytes\n\
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
            parse_type_declaration(advanced_bytes).unwrap().1,
            expected_result
        );
    }
}
