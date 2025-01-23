use chumsky::prelude::*;

/// Parse a base 10 32-bit integer.
pub fn int32() -> impl Parser<char, i32, Error = Simple<char>> {
    text::int(10).map(|s: String| s.parse().unwrap()).padded()
}
