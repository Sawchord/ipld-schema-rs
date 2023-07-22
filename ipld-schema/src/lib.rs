#![allow(dead_code, unused_variables)]

mod bytes;
mod comment;
mod enumerate;
mod parse;
mod representation;
mod unit;

use enumerate::EnumType;
use pest_derive::Parser;
use std::collections::BTreeMap;
use unit::UnitRepresentation;

#[derive(Parser)]
#[grammar = "schema.pest"]
pub struct SchemaParser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Doc<T> {
    doc: Option<String>,
    ty: T,
}

/// The toplevel schema represents a Ipld Data structure
/// mapping names
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpldSchema(BTreeMap<String, Doc<IpldType>>);

#[derive(Debug, Clone, PartialEq, Eq)]
enum IpldType {
    Bool,
    String,
    Bytes,
    Int,
    Float,
    // TODO: Map
    // TODO: List
    Link(String),
    // TODO: Union
    // TODO: Struct
    Enum(EnumType),
    Unit(UnitRepresentation),
    Any,
    Copy(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InlineIpldType {
    Name(String),
    // TODO: Map
    // TODO: List,
    Link(String),
}
