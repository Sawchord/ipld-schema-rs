use itertools::Itertools;
use pest::iterators::Pairs;
use thiserror::Error;

use crate::Rule;

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

// TODO: Move to list file
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListRepresentation {
    List,
    Advanced(String),
}

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringJoin(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringPairs {
    pub(crate) inner_delim: String,
    pub(crate) entry_delim: String,
}

pub(crate) fn parse_string_pairs(mut pair: Pairs<Rule>) -> StringPairs {
    let inner = pair.next().unwrap();
    assert!(pair.next().is_none());

    let (inner, entry) = inner.into_inner().collect_tuple().unwrap();
    assert_eq!(inner.as_rule(), Rule::stringpairs_repr_inner);
    assert_eq!(entry.as_rule(), Rule::stringpairs_repr_entry);

    StringPairs {
        inner_delim: parse_string_pairs_value(inner.into_inner()),
        entry_delim: parse_string_pairs_value(entry.into_inner()),
    }
}

fn parse_string_pairs_value(mut pairs: Pairs<Rule>) -> String {
    let inner = pairs.next().unwrap();
    assert!(pairs.next().is_none());
    assert_eq!(inner.as_rule(), Rule::stringpairs_repr_value);
    inner.as_str().to_string()
}

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Envelope {
    discriminant_key: String,
    content_key: String,
}
