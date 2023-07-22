use thiserror::Error;

use crate::{enumerate::InvalidEnum, IpldSchema};

#[derive(Debug, Clone, PartialEq, Eq, Error, Default)]
pub enum IpldSchemaParseError {
    #[error("Parsing error in comment block")]
    InvalidComment,
    #[error("{0}")]
    Enum(InvalidEnum),
    #[default]
    #[error("Unknown error")]
    Unknown,
}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        todo!()
    }
}
