use crate::enums::Expr;
use crate::parsers;
use chumsky::prelude::*;

/// Parse a string of characters into a random expression.
pub fn parse_random() -> impl Parser<char, Expr, Error = Simple<char>> {
    // Create parser to read a string

    // Create parser to read random list
    let random = parsers::string()
        .chain(just(',').ignore_then(parsers::string()).repeated())
        .or_not()
        .flatten()
        .delimited_by(just("#random {"), just('}'))
        .map(Expr::Random)
        .labelled("random array");

    random.padded()
}
