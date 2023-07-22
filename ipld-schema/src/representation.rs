use nom::{bytes::complete::tag, character::complete::space1, combinator::map, sequence::tuple};
use nom_diagnostic::{InStr, ParseResult};

use crate::parse::{parse_type_name, IpldSchemaParseError};

// TODO: Move to structured file
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StructRepresentation {
    Map,
    Tuple,
    StringPairs(StringPairs),
    StringJoin(StringJoin),
    ListPairs,
    Advanced(String),
}

// TODO: Move to map file
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapRepresentation {
    Map,
    StringPairs(StringPairs),
    ListPairs,
    Advanced(String),
}

// TODO: Move to list file
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListRepresentation {
    List,
    Advanced(String),
}

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringJoin(String);

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringPairs {
    inner_delim: String,
    entry_delim: String,
}

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Envelope {
    discriminant_key: String,
    content_key: String,
}

pub(crate) fn parse_advanced(input: InStr) -> ParseResult<InStr, IpldSchemaParseError> {
    map(
        tuple((tag("advanced"), space1, parse_type_name)),
        |(_, _, name)| name,
    )(input)
}

// TODO: Test StringJoin, StringPairs, Envelope
