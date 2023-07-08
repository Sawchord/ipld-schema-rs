mod comment;
mod enumerate;
mod primitives;

use self::{
    comment::parse_comment_block,
    enumerate::parse_enum,
    primitives::{
        parse_any, parse_bool, parse_bytes, parse_float, parse_int, parse_link, parse_string,
        parse_unit,
    },
};
use crate::{Doc, IpldSchema, IpldType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::{multispace0, multispace1},
    combinator::{map, opt, peek},
    multi::fold_many0,
    sequence::tuple,
    AsChar, Finish,
};
use nom_diagnostic::{ErrorDiagnose, IResult, InStr, ParseResult};
use std::collections::BTreeMap;
use thiserror::Error;

// TODO: Proper error handling

#[derive(Debug, Clone, PartialEq, Eq, Error, Default)]
pub enum IpldSchemaParseError {
    #[default]
    #[error("Unknown Error")]
    Unknown,
}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        let input = InStr::new(input.as_ref());
        parse_schema(input)
            .finish()
            .map(|(_, schema)| IpldSchema(schema))
            .map_err(|_| IpldSchemaParseError::Unknown)
    }
}

fn parse_schema(
    input: InStr,
) -> ParseResult<BTreeMap<String, Doc<IpldType>>, IpldSchemaParseError> {
    // TODO: Handle name duplication
    // TODO: How to handle non empty input
    ErrorDiagnose::compat(fold_many0(
        parse_type_declaration,
        BTreeMap::new,
        |mut schema, (name, decl)| {
            schema.insert(name, decl);
            schema
        },
    )(input))
}

/// Parses a complete type declaration, i.e. the type name and the type definiton
fn parse_type_declaration(input: InStr) -> IResult<(String, Doc<IpldType>)> {
    map(
        tuple((
            opt(parse_comment_block),
            multispace0,
            tag("type"),
            multispace1,
            parse_type_name,
            multispace1,
            parse_type_definition,
            multispace0,
        )),
        |parsed| {
            (
                parsed.4.to_string(),
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
fn parse_type_name(input: InStr) -> IResult<InStr> {
    map(
        tuple((
            peek(take_while1(|c: char| c.is_alpha() && c.is_uppercase())),
            take_till1(|c: char| !(c.is_alphanum() || c == '_')),
        )),
        |(_, x)| x,
    )(input)
}

/// Parses the type definition
fn parse_type_definition(input: InStr) -> IResult<IpldType> {
    alt((
        parse_bool,
        parse_string,
        parse_int,
        parse_float,
        parse_any,
        parse_bytes,
        parse_link,
        parse_unit,
        parse_enum,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type_name() {
        let test1 = "TypeName".into();
        let test2 = "Also_a_typename1232".into();
        let test3 = "_not_a_type_name".into();
        let test4 = "0notatypename".into();
        let test5 = "nottypenameeither".into();

        assert_eq!(parse_type_name(test1).unwrap().1.inner(), "TypeName");
        assert!(parse_type_name(test2).is_ok());
        assert!(parse_type_name(test3).is_err());
        assert!(parse_type_name(test4).is_err());
        assert!(parse_type_name(test5).is_err());
    }
}
