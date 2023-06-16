mod comment;
mod enumerate;
mod primitives;

use crate::{IpldSchema, IpldType};
use nom::{
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::multispace0,
    combinator::{map, peek},
    sequence::tuple,
    AsChar, IResult,
};

pub enum IpldSchemaParseError {}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        todo!()
    }
}

/// Parses a complete type declaration, i.e. the type name and the type definiton
fn parse_type_declaration(input: &str) -> IResult<&str, (String, IpldType)> {
    map(tuple((multispace0, tag("type"), multispace0)), |x| todo!())(input)
}

/// Parses the type definition
fn parse_type_definition(input: &str) -> IResult<&str, IpldType> {
    todo!()
}

/// Checks that a type name is correctly formed:
///
/// - First character is a capital letter
/// - Rest of the characters are alphanumerical or underscore
fn parse_type_name(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            peek(take_while1(|c: char| c.is_alpha() && c.is_uppercase())),
            take_till1(|c: char| !(c.is_alphanum() || c == '_')),
        )),
        |(_, x)| x,
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type_name() {
        let test1 = "TypeName";
        let test2 = "Also_a_typename1232";
        let test3 = "_not_a_type_name";
        let test4 = "0notatypename";
        let test5 = "nottypenameeither";

        assert_eq!(parse_type_name(test1).unwrap().1, "TypeName");
        assert!(parse_type_name(test2).is_ok());
        assert!(parse_type_name(test3).is_err());
        assert!(parse_type_name(test4).is_err());
        assert!(parse_type_name(test5).is_err());
    }
}
