#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ByteRepresentation {
    Bytes,
    Advances(String),
}
