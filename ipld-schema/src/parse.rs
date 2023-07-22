use itertools::Itertools;
use pest::{
    iterators::{Pair, Pairs},
    Parser,
};
use thiserror::Error;

use crate::{
    comment::parse_comment,
    enumerate::{parse_enum, InvalidEnum},
    IpldSchema, IpldType, Rule, SchemaParser,
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

        let mut current_comment = None;
        for pair in defs.into_inner() {
            match pair.as_rule() {
                Rule::comment => current_comment = Some(parse_comment(pair.into_inner())),
                Rule::r#type => {
                    let typ = parse_type(pair.into_inner())?;

                    todo!()
                }
                _ => todo!(),
            }
        }
        todo!()
    }
}

fn parse_type(def: Pairs<Rule>) -> Result<IpldType, IpldSchemaParseError> {
    let (name, decl) = def.collect_tuple().unwrap();

    assert_eq!(name.as_rule(), Rule::type_name);
    assert_eq!(decl.as_rule(), Rule::type_def);

    // Test whether we are having a primitive type
    match decl.as_str() {
        "bool" => return Ok(IpldType::Bool),
        "string" => return Ok(IpldType::String),
        "int" => return Ok(IpldType::Int),
        "float" => return Ok(IpldType::Float),
        "any" => return Ok(IpldType::Any),
        "bytes" => return Ok(IpldType::Bytes),
        _ => (),
    }

    let mut outer = decl.into_inner();
    let def = outer.next().unwrap();
    assert!(outer.next().is_none());

    match def.as_rule() {
        Rule::enum_def => parse_enum(def.into_inner()),
        Rule::link_def => todo!(),
        Rule::unit_def => todo!(),
        _ => todo!(),
    }
}
