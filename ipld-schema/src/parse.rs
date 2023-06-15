mod comment;
mod enumerate;
mod primitive;

use crate::IpldSchema;

pub enum IpldSchemaParseError {}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        todo!()
    }
}
