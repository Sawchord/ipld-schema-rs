#![allow(dead_code, unused_variables)]

mod comment;
mod parse;
mod representation;

use representation::{BytesRepresentation, EnumRepresentation, UnitRepresentation};
use std::collections::BTreeMap;

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
    Bytes(BytesRepresentation),
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
struct EnumType {
    members: Vec<Doc<String>>,
    representation: EnumRepresentation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InlineIpldType {
    Name(String),
    // TODO: Map
    // TODO: List,
    Link(String),
}
