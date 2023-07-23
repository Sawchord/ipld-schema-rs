use crate::{
    parse::{parse_inline_type, IpldSchemaParseError},
    representation::{parse_string_pairs, StringPairs},
    InlineIpldType, Rule,
};
use pest::iterators::Pairs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapType {
    pub(crate) key: String,
    pub(crate) value: InlineIpldType,
    pub(crate) nullable: bool,
    pub(crate) repr: MapRepresentation,
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
pub(crate) enum MapRepresentation {
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
    use crate::{list::ListType, IpldSchema, IpldType};
    use std::collections::BTreeMap;

    #[test]
    fn map() {
        let file = include_str!("../test/maps.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());

        expected_schema.0.insert(
            "SimpleMap".to_string(),
            crate::Doc {
                doc: Some("A simple map that maps one type to another".to_string()),
                ty: IpldType::Map(MapType {
                    key: "Int".to_string(),
                    value: InlineIpldType::Name("Float".to_string()),
                    nullable: false,
                    repr: MapRepresentation::Map,
                }),
            },
        );

        expected_schema.0.insert(
            "NullableLink".to_string(),
            crate::Doc {
                doc: Some(
                    "A composite type that maps one type to a Link that is also nullable"
                        .to_string(),
                ),
                ty: IpldType::Map(MapType {
                    key: "String".to_string(),
                    value: InlineIpldType::Link("Any".to_string()),
                    nullable: true,
                    repr: MapRepresentation::Map,
                }),
            },
        );

        expected_schema.0.insert(
            "MapOfLists".to_string(),
            crate::Doc {
                doc: Some(
                    "A composite map that is internally rerpesented as a pair of lists".to_string(),
                ),
                ty: IpldType::Map(MapType {
                    key: "String".to_string(),
                    value: InlineIpldType::List(Box::new(ListType {
                        ty: InlineIpldType::Name("Bool".to_string()),
                        nullable: true,
                    })),
                    nullable: false,
                    repr: MapRepresentation::ListPairs,
                }),
            },
        );

        expected_schema.0.insert(
            "MountOptions".to_string(),
            crate::Doc {
                doc: Some("A map that is represented as a String".to_string()),
                ty: IpldType::Map(MapType {
                    key: "String".to_string(),
                    value: InlineIpldType::Name("String".to_string()),
                    nullable: false,
                    repr: MapRepresentation::StringPairs(StringPairs {
                        inner_delim: "=".to_string(),
                        entry_delim: ",".to_string(),
                    }),
                }),
            },
        );

        assert_eq!(parsed_schema, expected_schema);
    }
}
