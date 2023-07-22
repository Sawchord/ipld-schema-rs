use crate::{
    parse::{parse_inline_type, IpldSchemaParseError},
    InlineIpldType, Rule,
};
use pest::iterators::Pairs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ListType {
    ty: InlineIpldType,
    nullable: bool,
}

pub(crate) fn parse_list(mut list: Pairs<Rule>) -> Result<ListType, IpldSchemaParseError> {
    let nullable = if list.peek().unwrap().as_rule() == Rule::list_nullable {
        let _ = list.next();
        true
    } else {
        false
    };

    let inner = list.next().unwrap();
    assert!(list.next().is_none());

    let ty = parse_inline_type(inner.into_inner())?;

    Ok(ListType { ty, nullable })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Doc, IpldSchema, IpldType};
    use std::collections::BTreeMap;

    #[test]
    fn list() {
        let file = include_str!("../test/list.ipldsch");

        let parsed_schema = IpldSchema::parse(file).unwrap();
        let mut expected_schema = IpldSchema(BTreeMap::new());

        expected_schema.0.insert(
            "SimpleList".to_string(),
            Doc {
                doc: Some("A list that is defined using another".to_string()),
                ty: IpldType::List(ListType {
                    ty: InlineIpldType::Name("String".to_string()),
                    nullable: false,
                }),
            },
        );
        expected_schema.0.insert(
            "NullableList".to_string(),
            Doc {
                doc: Some("A list that is nullable".to_string()),
                ty: IpldType::List(ListType {
                    ty: InlineIpldType::Name("String".to_string()),
                    nullable: true,
                }),
            },
        );
        expected_schema.0.insert(
            "ListOfLists".to_string(),
            Doc {
                doc: Some("A list of lists".to_string()),
                ty: IpldType::List(ListType {
                    ty: InlineIpldType::List(Box::new(ListType {
                        ty: InlineIpldType::Name("String".to_string()),
                        nullable: false,
                    })),
                    nullable: false,
                }),
            },
        );
        expected_schema.0.insert(
            "LinkList".to_string(),
            Doc {
                doc: Some("A list of links".to_string()),
                ty: IpldType::List(ListType {
                    ty: InlineIpldType::Link("String".to_string()),
                    nullable: false,
                }),
            },
        );

        assert_eq!(parsed_schema, expected_schema);
    }
}
