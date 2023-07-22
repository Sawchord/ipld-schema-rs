use crate::parse::{parse_type_name, IpldSchemaParseError};
use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1, not_line_ending, space0, space1},
    combinator::map,
    sequence::{delimited, tuple},
};
use nom_diagnostic::{map_diagnose, ErrorDiagnose, InStr, ParseResult, Span};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error, Default)]
pub enum RepresentationParseError {
    #[error("Stringpair representation fields are invalid")]
    StringPair,
    #[default]
    #[error("Representation is invalid")]
    Invalid,
}

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

pub(crate) fn parse_string_pairs(
    input: InStr,
) -> ParseResult<StringPairs, RepresentationParseError> {
    map(
        tuple((
            tag("stringpairs"),
            space0,
            delimited(tag("{"), parse_string_pairs_inner, tag("}")),
            multispace1,
        )),
        |(_, _, pairs, _)| pairs,
    )(input)
}

pub(crate) fn parse_string_pairs_inner(
    input: InStr,
) -> ParseResult<StringPairs, RepresentationParseError> {
    map_diagnose(
        tuple((multispace0, parse_kv, multispace1, parse_kv, multispace1)),
        |(_, (k1, v1), _, (k2, v2), _)| {
            let (inner_delim, entry_delim) = match (k1.as_str(), k2.as_str()) {
                ("innerDelim", "entryDelim") => (k1.to_string(), k2.to_string()),
                ("entryDelim", "innerDelim") => (k2.to_string(), k1.to_string()),
                ("innerDelim", _) => return string_pair_error(k2, "must be \"entryDelim\""),
                ("entryDelim", _) => return string_pair_error(k2, "must be \"innerDelim\""),
                (_, "entryDelim") => return string_pair_error(k1, "must be \"innerDelim\""),
                (_, "innerDelim") => return string_pair_error(k1, "must be \"entryDelim\""),
                (_, _) => {
                    return string_pair_error(
                        k1,
                        "Stringpairs is missing fields \"innerDelim\" and \"entryDelim\"",
                    );
                }
            };

            Ok::<_, ErrorDiagnose<'_, _>>(StringPairs {
                inner_delim,
                entry_delim,
            })
        },
    )(input)
}

fn string_pair_error<'a>(
    val: Span<'a, String>,
    hint: &'static str,
) -> Result<StringPairs, ErrorDiagnose<'a, RepresentationParseError>> {
    val.map(|_| Err(RepresentationParseError::StringPair))
        .with_hint(hint)
        .transform()
        .map_err(ErrorDiagnose::from)
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

/// Parses a key value pair, where the value is delimited by quotes
fn parse_kv(input: InStr) -> ParseResult<(Span<String>, Span<String>), RepresentationParseError> {
    map(
        tuple((
            not_line_ending,
            space1,
            delimited(tag("\""), not_line_ending, tag("\"")),
        )),
        |(key, _, val): (InStr, _, InStr)| (key.map(|s| s.to_string()), val.map(|s| s.to_string())),
    )(input)
}

// TODO: Test StringJoin, StringPairs, Envelope
