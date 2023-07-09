use crate::representation::{BytesRepresentation, UnitRepresentation};

use super::{enumerate::EnumRepresentationTag, parse_type_name, IpldSchemaParseError};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::tuple,
};
use nom_diagnostic::{diagnose, span, InStr, ParseResult, Span};

// TODO: Diagnose
pub(super) fn parse_bytes_representation(
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

pub(super) fn parse_enum_representation(
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
                                IpldSchemaParseError::InvalidEnumRepresentation(name.to_string())
                            },
                        )
                        .with_hint("this is not a valid value")
                },
            ),
        )),
        |(_, _, _, tag)| tag,
    ))(input)
}

// TODO: Error diagnse
pub(super) fn parse_unit_representation(
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

// TODO: Tests
