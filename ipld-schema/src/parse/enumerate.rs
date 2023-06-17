use super::comment::parse_comment_block;
use crate::IpldType;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_till1, take_while1},
    character::complete::{multispace0, multispace1, space0, space1},
    combinator::{map, opt, peek},
    sequence::tuple,
    AsChar, IResult,
};

pub(crate) fn parse_enum(input: &str) -> IResult<&str, IpldType> {
    todo!()
}

fn parse_enum_member(input: &str) -> IResult<&str, (Option<String>, String, EnumMemberTag)> {
    map(
        tuple((
            opt(parse_comment_block),
            multispace0,
            tag("|"),
            parse_enum_member_name,
            opt(parse_enum_member_tag),
            multispace1,
        )),
        |(comment, _, _, name, tag, _)| {
            (
                comment,
                name.to_string(),
                tag.unwrap_or(EnumMemberTag::String(name.to_string())),
            )
        },
    )(input)
}

enum EnumMemberTag {
    Int(i128),
    String(String),
}

fn parse_enum_member_tag(input: &str) -> IResult<&str, EnumMemberTag> {
    map(
        tuple((
            space1,
            tag("("),
            space0,
            tag("\""),
            space0,
            alt((
                map(parse_enum_member_name, |name| {
                    EnumMemberTag::String(name.to_string())
                }),
                map(nom::character::complete::i128, |int| {
                    EnumMemberTag::Int(int)
                }),
            )),
            space0,
            tag("\""),
            space0,
            tag(")"),
        )),
        |tag| tag.5,
    )(input)
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
