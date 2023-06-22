use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha0, alphanumeric0, alphanumeric1, digit0},
    combinator::{map, opt},
    multi::{fold_many1, many0, many1},
    sequence::{terminated, tuple},
    Finish,
};
use nom_diagnostic::{diagnose, ErrorDiagnose, IResult, InstrumentedStr, ParseResult};
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

#[derive(Debug, Clone, Error, Default)]
enum UrlParseError {
    #[error("protocol must be either '\"http\" or \"https\"")]
    InvalidProtocol,
    #[error("the domain must consist at least of a TLD and a subdomain")]
    InvalidDomain,
    #[error("ports must be in the range of 1 to 65536")]
    InvalidPort,
    #[error("unkown error")]
    #[default]
    Unknown,
}

impl Url {
    fn parse(input: &str) -> Result<Self, ErrorDiagnose<UrlParseError>> {
        let input = InstrumentedStr::new(input);
        let (rest, url) = parse_url(input).finish()?;
        rest.finalize(UrlParseError::InvalidPort)?;

        Ok(url)
    }
}

fn parse_url(input: InstrumentedStr) -> ParseResult<Url, UrlParseError> {
    map(
        tuple((
            parse_protocol,
            tag("://"),
            parse_domain,
            tag(":"),
            opt(parse_port),
        )),
        |(protocol, _, domain, _, port)| Url {
            protocol,
            domain,
            port,
        },
    )(input)
}

fn parse_protocol(input: InstrumentedStr) -> ParseResult<Protocol, UrlParseError> {
    diagnose(
        alt((
            map(tag("https"), |_: InstrumentedStr| Protocol::Https),
            map(tag("http"), |_: InstrumentedStr| Protocol::Http),
        )),
        alpha0,
        UrlParseError::InvalidProtocol,
    )(input)
}

fn parse_domain(input: InstrumentedStr) -> ParseResult<Domain, UrlParseError> {
    diagnose(
        map(
            fold_many1(
                parse_domain_level,
                Vec::new,
                |mut segments, segment: InstrumentedStr| {
                    segments.push(segment.inner().to_string());
                    segments
                },
            ),
            |vec| Domain(vec),
        ),
        //many1(alt((tag("."), alphanumeric1))),
        alphanumeric0,
        UrlParseError::InvalidDomain,
    )(input)
}

fn parse_domain_level(input: InstrumentedStr) -> IResult<InstrumentedStr> {
    map(tuple((alphanumeric1, opt(tag(".")))), |(domain, x)| {
        dbg!(&domain, x.map(|x: InstrumentedStr| x.inner()));
        domain
    })(input)
}

fn parse_port(input: InstrumentedStr) -> ParseResult<u16, UrlParseError> {
    diagnose(
        nom::character::complete::u16,
        digit0,
        UrlParseError::InvalidPort,
    )(input)
}

fn main() {
    match Url::parse("https://test.example.com:8080") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }
}
