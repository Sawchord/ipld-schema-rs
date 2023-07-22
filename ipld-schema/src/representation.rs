use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum BytesRepresentation {
    Bytes,
    Advanced(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StructRepresentation {
    Map,
    Tuple,
    StringPairs(StringPairs),
    StringJoin(StringJoin),
    ListPairs,
    Advanced(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapRepresentation {
    Map,
    StringPairs(StringPairs),
    ListPairs,
    Advanced(String),
}

// TODO: Define UnitRepresentation

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ListRepresentation {
    List,
    Advanced(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString)]
pub(crate) enum EnumRepresentation {
    String(Vec<String>),
    Int(Vec<i128>),
}

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum UnitRepresentation {
    Null,
    True,
    False,
    EmptyMap,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringJoin(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StringPairs {
    inner_delim: String,
    entry_delim: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Envelope {
    discriminant_key: String,
    content_key: String,
}
