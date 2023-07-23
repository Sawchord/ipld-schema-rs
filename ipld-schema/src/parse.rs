use std::collections::BTreeMap;

use itertools::Itertools;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use thiserror::Error;

use crate::{
    comment::parse_comment,
    enumerate::{parse_enum, InvalidEnum},
    list::parse_list,
    map::parse_map,
    structural::parse_struct,
    unit::parse_unit,
    Doc, InlineIpldType, IpldSchema, IpldType, Rule, SchemaParser,
};

#[derive(Debug, Clone, PartialEq, Eq, Error, Default)]
pub enum IpldSchemaParseError {
    #[error("{0}")]
    Enum(InvalidEnum),
    #[default]
    #[error("Unknown error")]
    Unknown,
}

impl IpldSchema {
    pub fn parse(input: impl AsRef<str>) -> Result<Self, IpldSchemaParseError> {
        // TODO: Error output
        let mut outer = SchemaParser::parse(Rule::file, input.as_ref()).unwrap();
        let defs: Pair<_> = outer.next().unwrap();
        assert!(outer.next().is_none());

        dbg!(&defs);

        let mut definitions = BTreeMap::new();

        let mut current_comment = None;
        for pair in defs.into_inner() {
            match pair.as_rule() {
                Rule::comment => current_comment = Some(parse_comment(pair.into_inner())),
                Rule::r#type => {
                    let (name, ty) = parse_type(pair.into_inner())?;

                    definitions.insert(
                        name,
                        Doc {
                            doc: current_comment.take(),
                            ty,
                        },
                    );
                }
                Rule::EOI => (),
                _ => todo!(),
            }
        }

        Ok(Self(definitions))
    }
}

fn parse_type(def: Pairs<Rule>) -> Result<(String, IpldType), IpldSchemaParseError> {
    let (name, decl) = def.collect_tuple().unwrap();

    assert_eq!(name.as_rule(), Rule::type_name);
    assert_eq!(decl.as_rule(), Rule::type_def);

    let name = name.as_str().to_string();

    // Test whether we are having a primitive type
    match decl.as_str() {
        "bool" => return Ok((name, IpldType::Bool)),
        "string" => return Ok((name, IpldType::String)),
        "int" => return Ok((name, IpldType::Int)),
        "float" => return Ok((name, IpldType::Float)),
        "any" => return Ok((name, IpldType::Any)),
        "bytes" => return Ok((name, IpldType::Bytes)),
        _ => (),
    }

    let mut outer = decl.into_inner();
    let def = outer.next().unwrap();
    assert!(outer.next().is_none());

    match def.as_rule() {
        Rule::list_def => Ok((name, IpldType::List(parse_list(def.into_inner())?))),
        Rule::map_def => Ok((name, IpldType::Map(parse_map(def.into_inner())?))),
        Rule::struct_def => Ok((name, IpldType::Struct(parse_struct(def.into_inner())?))),
        Rule::enum_def => Ok((name, IpldType::Enum(parse_enum(def.into_inner())?))),
        Rule::link_def => Ok((name, IpldType::Link(parse_link(def.into_inner())?))),
        Rule::unit_def => Ok((name, parse_unit(def.into_inner())?)),
        _ => todo!(),
    }
}

pub(crate) fn parse_inline_type(
    mut tok: Pairs<Rule>,
) -> Result<InlineIpldType, IpldSchemaParseError> {
    let inner = tok.next().unwrap();
    assert!(tok.next().is_none());

    match inner.as_rule() {
        Rule::type_name => Ok(InlineIpldType::Name(inner.as_str().to_string())),
        Rule::list_def => Ok(InlineIpldType::List(Box::new(parse_list(
            inner.into_inner(),
        )?))),
        Rule::map_def => Ok(InlineIpldType::Map(Box::new(parse_map(
            inner.into_inner(),
        )?))),
        Rule::link_def => Ok(InlineIpldType::Link(parse_link(inner.into_inner())?)),
        _ => panic!(),
    }
}

fn parse_link(mut link: Pairs<Rule>) -> Result<String, IpldSchemaParseError> {
    let inner = link.next().unwrap();
    assert!(link.next().is_none());
    assert_eq!(inner.as_rule(), Rule::type_name);
    Ok(inner.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use crate::unit::UnitRepresentation;

    use super::*;

    #[test]
    fn primitives() {
        let file = include_str!("../test/primitive.ipldsch");
        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());

        expected_schema.0.insert(
            "TestString".to_string(),
            Doc {
                doc: Some("This string is documented\nSkipping a line".to_string()),
                ty: IpldType::String,
            },
        );
        expected_schema.0.insert(
            "TestInt".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Int,
            },
        );
        expected_schema.0.insert(
            "TestLink".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Link("Link".to_string()),
            },
        );
        expected_schema.0.insert(
            "NullUnit".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Unit(UnitRepresentation::Null),
            },
        );

        assert_eq!(parsed_schema, expected_schema);
    }
}
