mod comment;
mod enumerate;
mod primitives;

use crate::{Doc, IpldSchema, IpldType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::multispace0,
    combinator::{map, opt, peek},
    sequence::tuple,
    AsChar, IResult,
};

use self::{
    comment::parse_comment_block,
    primitives::{parse_any, parse_bool},
};

pub enum IpldSchemaParseError {}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        todo!()
    }
}

/// Parses a complete type declaration, i.e. the type name and the type definiton
fn parse_type_declaration(input: &str) -> IResult<&str, (String, Doc<IpldType>)> {
    map(
        tuple((
            opt(parse_comment_block),
            multispace0,
            tag("type"),
            multispace0,
            parse_type_name,
            multispace0,
            parse_type_definition,
            multispace0,
        )),
        |parsed| {
            (
                String::from(parsed.4),
                Doc {
                    doc: parsed.0,
                    ty: parsed.6,
                },
            )
        },
    )(input)
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

/// Parses the type definition
fn parse_type_definition(input: &str) -> IResult<&str, IpldType> {
    alt((parse_bool, parse_any))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bool_declaration_uncommented() {
        let uncommented_bool = "type UncommentedBool bool";
        let expected_result = (
            String::from("UncommentedBool"),
            Doc {
                doc: None,
                ty: IpldType::Bool,
            },
        );

        assert_eq!(
            parse_type_declaration(uncommented_bool).unwrap().1,
            expected_result
        );
    }

    #[test]
    fn test_any_declaration_commented() {
        let commented_any = "\
            ## This is the documentation of this type\n\
            ##  \n\n\
            type Commented_Any any  \n";

        let expected_doc = "\
            This is the documentation of this type\n\
            \n\
        ";
        let expected_result = (
            String::from("Commented_Any"),
            Doc {
                doc: Some(String::from(expected_doc)),
                ty: IpldType::Any,
            },
        );

        assert_eq!(
            parse_type_declaration(commented_any).unwrap().1,
            expected_result
        );
    }

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
