use super::{parse_type_name, IpldSchemaParseError};
use crate::representation::UnitRepresentation;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

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

pub(crate) fn parse_advanced(input: InStr) -> ParseResult<InStr, IpldSchemaParseError> {
    map(
        tuple((tag("advanced"), space1, parse_type_name)),
        |(_, _, name)| name,
    )(input)
}

// TODO: Tests
