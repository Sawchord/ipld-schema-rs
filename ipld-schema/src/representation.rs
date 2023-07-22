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

// TODO: Parser
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Envelope {
    discriminant_key: String,
    content_key: String,
}
