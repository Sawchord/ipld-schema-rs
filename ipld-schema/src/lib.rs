#![allow(dead_code, unused_variables)]

mod comment;
mod enumerate;
mod list;
mod map;
mod parse;
mod representation;
mod unit;

use enumerate::EnumType;
use list::ListType;
use map::MapType;
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
pub(crate) enum IpldType {
    Bool,
    String,
    Bytes,
    Int,
    Float,
    List(ListType),
    Map(MapType),
    Link(String),
    // TODO: Union
    // TODO: Struct
    Enum(EnumType),
    Unit(UnitRepresentation),
    Any,
    Copy(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum InlineIpldType {
    Name(String),
    // TODO: Map
    List(Box<ListType>),
    Link(String),
}
