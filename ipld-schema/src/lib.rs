#![allow(dead_code)]

mod representation;

use std::collections::BTreeMap;

/// The toplevel schema represents a Ipld Data structure
/// mapping names
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IpldSchema(BTreeMap<String, DocIpldType>);

/// A type declaration, optionally annotated by a doc block
#[derive(Debug, Clone, PartialEq, Eq)]
struct DocIpldType {
    doc: Option<String>,
    ty: IpldType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IpldType {
    Bool,
    String,
    // TODO: Bytes
    Int,
    Float,
    // TODO: Map
    // TODO: List
    // TODO: Link
    // TODO: Union
    // TODO: Struct
    // TODO: Enum
    // TODO: Unit
    // TODO: Any
    // TODO: Copy
}
