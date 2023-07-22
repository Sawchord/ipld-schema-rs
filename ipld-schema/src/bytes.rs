use crate::{parse::IpldSchemaParseError, representation::parse_advanced, IpldType};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{map, opt},
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

pub(crate) fn parse_bytes(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("bytes"), opt(parse_bytes_representation))),
        |(_, repr)| IpldType::Bytes(repr.unwrap_or(BytesRepresentation::Bytes)),
    )(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BytesRepresentation {
    Bytes,
    Advanced(String),
}

// TODO: Diagnose
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

#[cfg(test)]
mod tests {
    use crate::{parse::parse_type_declaration, Doc};

    use super::*;

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
}
