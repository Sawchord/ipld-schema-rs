use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ByteRepresentation {
    Bytes,
    Advanced(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapRepresentation {
    StringPairs {
        inner_delim: String,
        outer_delim: String,
    },
    ListPairs,
    Advanced(String),
}

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
