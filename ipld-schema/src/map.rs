use crate::{
    parse::{parse_inline_type, IpldSchemaParseError},
    representation::{parse_string_pairs, StringPairs},
    InlineIpldType, Rule,
};
use pest::iterators::Pairs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapType {
    key: String,
    value: InlineIpldType,
    nullable: bool,
    repr: MapRepresentation,
}

pub(crate) fn parse_map(mut map: Pairs<Rule>) -> Result<MapType, IpldSchemaParseError> {
    let key: pest::iterators::Pair<'_, Rule> = map.next().unwrap();
    assert_eq!(key.as_rule(), Rule::type_name);
    let key = key.as_str().to_string();

    let nullable = if map.peek().unwrap().as_rule() == Rule::map_nullable {
        let _ = map.next().unwrap();
        true
    } else {
        false
    };

    let value = map.next().unwrap();
    assert_eq!(value.as_rule(), Rule::inline_type_def);
    let value = parse_inline_type(value.into_inner())?;

    let repr = if let Some(repr) = map.next() {
        assert!(map.next().is_none());
        parse_map_representation(repr.into_inner())
    } else {
        MapRepresentation::Map
    };

    Ok(MapType {
        key,
        value,
        nullable,
        repr,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MapRepresentation {
    Map,
    StringPairs(StringPairs),
    ListPairs,
}

fn parse_map_representation(mut repr: Pairs<Rule>) -> MapRepresentation {
    let inner = repr.next().unwrap();
    assert!(repr.next().is_none());

    match inner.as_str() {
        "map" => return MapRepresentation::Map,
        "listpairs" => return MapRepresentation::ListPairs,
        _ => (),
    }

    // In this case, it can only be a stringpairs
    let mut inner = inner.into_inner();
    let rule = inner.next().unwrap();
    assert!(inner.next().is_none());

    assert_eq!(rule.as_rule(), Rule::stringpairs_repr);
    MapRepresentation::StringPairs(parse_string_pairs(rule.into_inner()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::IpldSchema;
    use std::collections::BTreeMap;

    #[test]
    fn map() {
        let file = include_str!("../test/maps.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());

        assert_eq!(parsed_schema, expected_schema);
    }
}
