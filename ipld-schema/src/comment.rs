use crate::Rule;
use itertools::Itertools;
use pest::iterators::Pairs;

pub(crate) fn parse_comment(comment: Pairs<Rule>) -> String {
    comment
        .map(|pair| pair.as_str().trim().to_string())
        .join("\n")
}
