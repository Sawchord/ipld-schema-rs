use crate::Rule;
use pest::iterators::Pairs;

pub(crate) fn parse_comment(comment: Pairs<Rule>) -> String {
    let mut lines: Vec<String> = vec![];

    for pair in comment {
        assert_eq!(pair.as_rule(), Rule::comment_line);
        let mut inner = pair.into_inner();
        let comment = inner.next().unwrap();
        assert!(inner.next().is_none());
        lines.push(comment.as_str().trim().to_string());
    }

    lines.join("\n")
}
