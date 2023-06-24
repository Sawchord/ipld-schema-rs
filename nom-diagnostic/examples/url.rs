use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::alphanumeric1,
    combinator::{map, opt},
    multi::fold_many1,
    sequence::tuple,
    Finish,
};
use nom_diagnostic::{diagnose, ErrorDiagnose, IResult, InStr, ParseResult};
use thiserror::Error;

#[derive(Debug, Clone)]
enum Protocol {
    Http,
    Https,
}

#[derive(Debug, Clone)]
struct Domain(Vec<String>);

#[derive(Debug, Clone)]
#[allow(dead_code)]
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
        let input = InStr::new(input);
        let (rest, url) = parse_url(input).finish()?;
        rest.finalize(
            UrlParseError::InvalidPort,
            "port number can only be a decimal number between 1 and 65536",
        )?;

        Ok(url)
    }
}

fn parse_url(input: InStr) -> ParseResult<Url, UrlParseError> {
    map(
        tuple((parse_protocol, tag("://"), parse_domain, opt(parse_port))),
        |(protocol, _, domain, port)| Url {
            protocol,
            domain,
            port,
        },
    )(input)
}

fn parse_protocol(input: InStr) -> ParseResult<Protocol, UrlParseError> {
    diagnose(
        alt((
            map(tag("https"), |_: InStr| Protocol::Https),
            map(tag("http"), |_: InStr| Protocol::Http),
        )),
        |error| {
            vec![error
                .input
                .to_span(|c| !c.is_alphanumeric(), UrlParseError::InvalidProtocol)
                .with_hint("this must be either \"http\" or \"https\"")]
        },
    )(input)
}

fn parse_domain(input: InStr) -> ParseResult<Domain, UrlParseError> {
    diagnose(
        map(
            fold_many1(
                parse_domain_level,
                Vec::new,
                |mut segments, segment: InStr| {
                    dbg!(&segment.inner());
                    segments.push(segment.inner().to_string());
                    segments
                },
            ),
            |vec| Domain(vec),
        ),
        |error| {
            vec![error
                .input
                .to_span(|c| !c.is_alphanumeric(), UrlParseError::InvalidDomain)
                .with_hint("domain must contain at least two segments, i.e. \"example.com\"")]
        },
    )(input)
}

fn parse_domain_level(input: InStr) -> IResult<InStr> {
    map(tuple((alphanumeric1, opt(tag(".")))), |(domain, _)| domain)(input)
}

fn parse_port(input: InStr) -> ParseResult<u16, UrlParseError> {
    map(
        tuple((tag(":"), nom::character::complete::u16)),
        |(_, port)| port,
    )(input)
}

fn main() {
    match Url::parse("https://test.example.com") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }

    match Url::parse("htt://test.example.com:8080") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }

    match Url::parse("http://com:8080") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }

    match Url::parse("http://test.example.com:lol") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }

    match Url::parse("http://:8080") {
        Ok(url) => println!("{:?}", url),
        Err(err) => err.display(),
    }
}
