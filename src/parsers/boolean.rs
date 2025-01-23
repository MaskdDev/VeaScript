use chumsky::prelude::*;

/// Parse a boolean.
pub fn parse_bool() -> impl Parser<char, bool, Error = Simple<char>> {
    just("true")
        .or(just("false"))
        .padded()
        .map(|boolean| boolean == "true")
}
