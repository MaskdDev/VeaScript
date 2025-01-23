use chumsky::prelude::*;

/// Parse a hexadecimal value starting with # or 0x.
pub fn hexadecimal() -> impl Parser<char, i32, Error = Simple<char>> {
    just("0x")
        .or(just("#"))
        .ignore_then(text::int(16))
        .map(|s: String| i32::from_str_radix(&s, 16).unwrap())
}
