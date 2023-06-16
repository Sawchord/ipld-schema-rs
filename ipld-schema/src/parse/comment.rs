use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, not_line_ending, space0},
    combinator::{map, opt},
    multi::many1,
    sequence::tuple,
    IResult,
};

/// Parses a single line of comments that begins with `##` and ends with a newline
fn parse_comment_line(input: &str) -> IResult<&str, &str> {
    map(
        tuple((space0, tag("##"), space0, not_line_ending, line_ending)),
        |(_, _, _, data, _)| data,
    )(input)
}

/// Parses a comment block. Each line has to be either a comment or an empty line
pub(crate) fn parse_comment_block(input: &str) -> IResult<&str, String> {
    map(
        many1(tuple((parse_comment_line, opt(multispace0)))),
        |lines| {
            lines
                .into_iter()
                .map(|(line, _)| String::from(line))
                .fold(String::new(), |a, b| a + &b + "\n")
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comment_line() {
        let comment = "    ## This is a comment line\n";
        let not_comment = "This is not a comment";

        assert!(parse_comment_line(comment).is_ok());
        assert!(parse_comment_line(not_comment).is_err());
    }

    #[test]
    fn test_parse_comment_block() {
        let comment = "\
            ## This is a comment block\n\
            ## It starts with ## in the beginning\n\
               \n\
            ## Empty lines are not a problem for it\n\
            This is no longer a comment\n\
        ";

        let parsed_comment = "\
            This is a comment block\n\
            It starts with ## in the beginning\n\
            Empty lines are not a problem for it\n\
        ";

        let parsed = parse_comment_block(comment).unwrap().1;
        assert_eq!(parsed, parsed_comment)
    }
}
