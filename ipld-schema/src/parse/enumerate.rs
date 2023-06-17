use crate::{representation::EnumRepresentation, IpldType};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    combinator::{map, peek},
    sequence::tuple,
    AsChar, IResult,
};

pub(crate) fn parse_enum(input: &str) -> IResult<&str, IpldType> {
    todo!()
}

enum EnumMemberTag {
    Int(i128),
    String(String),
}

fn parse_enum_member_tag(input: &str) -> IResult<&str, EnumMemberTag> {
    alt((
        map(parse_enum_member_name, |name| {
            EnumMemberTag::String(name.to_string())
        }),
        map(nom::character::complete::i128, |int| {
            EnumMemberTag::Int(int)
        }),
    ))(input)
}

enum EnumRepresentationTag {
    Int,
    String,
}

fn parse_enum_representation_tag(input: &str) -> IResult<&str, EnumRepresentationTag> {
    alt((
        map(tag("int"), |_| EnumRepresentationTag::Int),
        map(tag("string"), |_| EnumRepresentationTag::String),
    ))(input)
}

fn parse_enum_member_name(input: &str) -> IResult<&str, &str> {
    map(
        tuple((
            peek(take_while1(|c: char| c.is_alpha())),
            take_till1(|c: char| !(c.is_alphanum() || c == '_')),
        )),
        |(_, x)| x,
    )(input)
}
