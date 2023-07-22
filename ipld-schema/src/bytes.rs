use crate::parse::{representation::parse_advanced, IpldSchemaParseError};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BytesRepresentation {
    Bytes,
    Advanced(String),
}

// TODO: Diagnose
pub(crate) fn parse_bytes_representation(
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
