use super::{parse_type_name, IpldSchemaParseError};

use nom::{bytes::complete::tag, character::complete::space1, combinator::map, sequence::tuple};
use nom_diagnostic::{InStr, ParseResult};

pub(crate) fn parse_advanced(input: InStr) -> ParseResult<InStr, IpldSchemaParseError> {
    map(
        tuple((tag("advanced"), space1, parse_type_name)),
        |(_, _, name)| name,
    )(input)
}

// TODO: Tests
