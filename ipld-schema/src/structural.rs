use pest::iterators::Pairs;

use crate::{
    comment::parse_comment,
    parse::{parse_inline_type, IpldSchemaParseError},
    representation::{parse_string_pairs, StringPairs},
    InlineIpldType, Rule,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct StructType {
    fields: Vec<StructField>,
    repr: StructRepresentation,
}

pub(crate) fn parse_struct(stru: Pairs<Rule>) -> Result<StructType, IpldSchemaParseError> {
    let mut fields = vec![];
    let mut repr = None;

    for pair in stru {
        match pair.as_rule() {
            Rule::struct_field => fields.push(parse_struct_field(pair.into_inner())?),
            Rule::struct_repr => repr = Some(parse_struct_representation(pair.into_inner())),
            _ => panic!(),
        }
    }

    Ok(StructType {
        fields,
        repr: repr.unwrap_or(StructRepresentation::Map),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct StructField {
    doc: Option<String>,
    key: String,
    value: InlineIpldType,
    optional: bool,
    nullable: bool,
    rename: Option<String>,
    implicit: Option<String>,
}

fn parse_struct_field(mut field: Pairs<Rule>) -> Result<StructField, IpldSchemaParseError> {
    let doc = if field.peek().unwrap().as_rule() == Rule::comment {
        let comment = field.next().unwrap();
        Some(parse_comment(comment.into_inner()))
    } else {
        None
    };

    let name = field.next().unwrap();
    assert_eq!(name.as_rule(), Rule::struct_field_name);
    let key = name.as_str().to_string();

    let nullable = if field.peek().unwrap().as_rule() == Rule::struct_nullable {
        let _ = field.next().unwrap();
        true
    } else {
        false
    };

    let optional = if field.peek().unwrap().as_rule() == Rule::struct_optional {
        let _ = field.next().unwrap();
        true
    } else {
        false
    };

    let value = field.next().unwrap();
    assert_eq!(value.as_rule(), Rule::inline_type_def);
    let value = parse_inline_type(value.into_inner())?;

    let (rename, implicit) = if let Some(repr) = field.next() {
        assert!(field.next().is_none());
        parse_struct_field_representation(repr.into_inner())
    } else {
        (None, None)
    };

    Ok(StructField {
        doc,
        key,
        value,
        optional,
        nullable,
        rename,
        implicit,
    })
}

fn parse_struct_field_representation(mut repr: Pairs<Rule>) -> (Option<String>, Option<String>) {
    let rename = if repr.peek().unwrap().as_rule() == Rule::struct_field_repr_rename {
        let rename = repr.next().unwrap();

        let mut inner = rename.into_inner();
        let rename = inner.next().unwrap();
        assert!(inner.next().is_none());
        assert_eq!(rename.as_rule(), Rule::struct_field_name);

        Some(rename.as_str().to_string())
    } else {
        None
    };

    let implicit = if let Some(implicit) = repr.next() {
        let mut inner = implicit.into_inner();
        let implicit = inner.next().unwrap();
        assert!(inner.next().is_none());
        assert_eq!(implicit.as_rule(), Rule::struct_field_name);
        assert!(repr.next().is_none());

        Some(implicit.as_str().to_string())
    } else {
        None
    };

    assert!(repr.next().is_none());

    (rename, implicit)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StructRepresentation {
    Map,
    ListPairs,
    StringPairs(StringPairs),
}

fn parse_struct_representation(mut repr: Pairs<Rule>) -> StructRepresentation {
    let inner = repr.next().unwrap();
    assert!(repr.next().is_none());

    match inner.as_str() {
        "map" => return StructRepresentation::Map,
        "listpairs" => return StructRepresentation::ListPairs,
        _ => (),
    }

    // In this case, it can only be a stringpairs
    let mut inner = inner.into_inner();
    let rule = inner.next().unwrap();
    assert!(inner.next().is_none());

    assert_eq!(rule.as_rule(), Rule::stringpairs_repr);
    StructRepresentation::StringPairs(parse_string_pairs(rule.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IpldSchema;
    use std::collections::BTreeMap;

    #[test]
    fn structural() {
        let file = include_str!("../test/struct.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());

        assert_eq!(parsed_schema, expected_schema);
    }
}
