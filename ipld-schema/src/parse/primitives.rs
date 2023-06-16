use crate::IpldType;
use nom::{bytes::complete::tag, combinator::map, IResult};

pub(crate) fn parse_bool(input: &str) -> IResult<&str, IpldType> {
    map(tag("bool"), |_| IpldType::Bool)(input)
}

// TODO: String
// TODO: Bytes
// TODO: Int
// TODO: Float

pub(crate) fn parse_any(input: &str) -> IResult<&str, IpldType> {
    map(tag("any"), |_| IpldType::Any)(input)
}

// TODO: Unit
