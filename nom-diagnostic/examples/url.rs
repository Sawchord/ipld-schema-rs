use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, digit0},
    combinator::map,
    sequence::tuple,
};
use nom_diagnostic::{diagnose, ErrorDiagnose, InstrumentedStr, ParseResult};
use thiserror::Error;

#[derive(Debug, Clone)]
enum Protocol {
    Http,
    Https,
}

#[derive(Debug, Clone)]
struct Domain(Vec<String>);

#[derive(Debug, Clone)]
struct Url {
    protocol: Protocol,
    domain: Domain,
    port: Option<u16>,
}

#[derive(Debug, Clone, Error)]
enum UrlParseError {
    #[error("protocol must be either '\"http\" or \"https\"")]
    InvalidProtocol,
    #[error("the domain must consist at least of a TLD and a subdomain")]
    InvalidDomain,
    #[error("ports must be in the range of 1 to 65536")]
    InvalidPort,
}

impl Url {
    fn parse(input: &str) -> Result<Self, ErrorDiagnose<UrlParseError>> {
        todo!()
    }
}

fn parse_protocol(input: InstrumentedStr) -> ParseResult<Protocol, UrlParseError> {
    diagnose(
        alt((
            map(tag("http"), |_: InstrumentedStr| Protocol::Http),
            map(tag("https"), |_: InstrumentedStr| Protocol::Https),
        )),
        alpha0,
        UrlParseError::InvalidProtocol,
    )(input)
}

fn parse_port(input: InstrumentedStr) -> ParseResult<u16, UrlParseError> {
    diagnose(
        nom::character::complete::u16,
        digit0,
        UrlParseError::InvalidPort,
    )(input)
}

fn main() {
    print!("This will be an example");
}
