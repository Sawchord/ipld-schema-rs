use pest::iterators::Pairs;

use crate::{parse::IpldSchemaParseError, IpldType, Rule};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UnitRepresentation {
    Null,
    True,
    False,
    EmptyMap,
}

pub(crate) fn parse_unit(mut unit: Pairs<Rule>) -> Result<IpldType, IpldSchemaParseError> {
    let inner = unit.next().unwrap();
    assert!(unit.next().is_none());
    assert_eq!(inner.as_rule(), Rule::unit_repr);

    let repr = match inner.as_str() {
        "null" => UnitRepresentation::Null,
        "false" => UnitRepresentation::False,
        "true" => UnitRepresentation::True,
        "emptymap" => UnitRepresentation::EmptyMap,
        _ => panic!(),
    };

    Ok(IpldType::Unit(repr))
}
