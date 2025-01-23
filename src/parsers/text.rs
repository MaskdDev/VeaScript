use crate::enums::Expr;
use chumsky::prelude::*;

/// Parse a VeaScript text block.
pub fn parse_text() -> impl Parser<char, Expr, Error = Simple<char>> {
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('b').to('\x08'))
            .or(just('f').to('\x0C'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t'))
            .or(just('u').ignore_then(
                filter(|c: &char| c.is_digit(16))
                    .repeated()
                    .exactly(4)
                    .collect::<String>()
                    .validate(|digits, span, emit| {
                        char::from_u32(u32::from_str_radix(&digits, 16).unwrap()).unwrap_or_else(
                            || {
                                emit(Simple::custom(span, "invalid unicode character"));
                                '\u{FFFD}' // unicode replacement character
                            },
                        )
                    }),
            )),
    );

    // Create parser to read text
    let text = filter(|c| *c != '}' && *c != '\\')
        .or(escape)
        .repeated()
        .or_not()
        .flatten()
        .delimited_by(just("#text {"), just('}'))
        .collect::<String>()
        .map(Expr::Text)
        .labelled("text");

    text.padded()
}
