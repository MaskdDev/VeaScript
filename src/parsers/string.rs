use chumsky::prelude::*;

/// Parse a string.
pub fn parse_string() -> impl Parser<char, String, Error = Simple<char>> {
    just("\"")
        .ignore_then(filter(|c| *c != '\\' && *c != '\"').repeated())
        .then_ignore(just("\""))
        .padded()
        .collect::<String>()
}
