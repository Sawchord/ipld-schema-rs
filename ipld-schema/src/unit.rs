use crate::{parse::IpldSchemaParseError, IpldType};
use nom::{
    branch::alt, bytes::complete::tag, character::complete::space1, combinator::map,
    sequence::tuple,
};
use nom_diagnostic::{InStr, ParseResult};

pub(crate) fn parse_unit(input: InStr) -> ParseResult<IpldType, IpldSchemaParseError> {
    map(
        tuple((tag("unit"), parse_unit_representation)),
        |(_, repr)| IpldType::Unit(repr),
    )(input)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum UnitRepresentation {
    Null,
    True,
    False,
    EmptyMap,
}

// TODO: Error diagnse
fn parse_unit_representation(
    input: InStr,
) -> ParseResult<UnitRepresentation, IpldSchemaParseError> {
    map(
        tuple((
            space1,
            tag("representation"),
            space1,
            alt((
                map(tag("null"), |_| UnitRepresentation::Null),
                map(tag("true"), |_| UnitRepresentation::True),
                map(tag("false"), |_| UnitRepresentation::False),
                map(tag("emptymap"), |_| UnitRepresentation::EmptyMap),
            )),
        )),
        |(_, _, _, repr)| repr,
    )(input)
}

#[cfg(test)]
mod tests {
    use crate::{parse::parse_type_declaration, Doc};

    use super::*;

    #[test]
    fn test_unit() {
        let unit = "type MyUnit unit representation true";
        let expexted_result = (
            "MyUnit".to_string(),
            Doc {
                doc: None,
                ty: IpldType::Unit(UnitRepresentation::True),
            },
        );
        assert_eq!(
            parse_type_declaration(unit.into()).unwrap().1,
            expexted_result
        );
    }
}
