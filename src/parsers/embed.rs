use crate::enums::{EmbedComponent, Expr};
use crate::helpers::hexadecimal;
use crate::parsers;
use crate::structs::StoredEmbed;
use chumsky::prelude::*;

/// Parse an embed into a VeaScript embed.
pub fn parse_embed() -> impl Parser<char, Expr, Error = Simple<char>> {
    // Return embed parser
    parse_embed_raw().map(Expr::Embed)
}

/// Parse an embed into a vector of embed components.
pub fn parse_embed_raw() -> impl Parser<char, Vec<EmbedComponent>, Error = Simple<char>> {
    // Create embed parser
    let embed_parser = parse_embed_component()
        .repeated()
        .delimited_by(just("#embed {"), just("}"))
        .collect();

    // Return embed parser
    embed_parser.padded()
}

/// Parse an embed component.
pub fn parse_embed_component() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Create embed component parser
    let component_parser = parse_embed_title()
        .or(parse_embed_description())
        .or(parse_embed_colour());

    // Return embed component parser
    component_parser.padded()
}

/// Parse an embed title.
pub fn parse_embed_title() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    let title_parser = parsers::string()
        .delimited_by(just("#title:"), just(','))
        .map(EmbedComponent::Title);

    title_parser
}

/// Parse an embed description
pub fn parse_embed_description() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Create parser to read a string
    let string = just("\"")
        .ignore_then(filter(|c| *c != '\\' && *c != '\"').repeated())
        .then_ignore(just("\""))
        .padded()
        .collect::<String>();

    let description_parser = string
        .clone()
        .delimited_by(just("#description:"), just(','))
        .map(EmbedComponent::Description);

    description_parser
}

/// Parse an embed colour
pub fn parse_embed_colour() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Create parser to read a hexadecimal
    let hex = just("0x")
        .or(just("#"))
        .ignore_then(text::int(16))
        .map(|s: String| i32::from_str_radix(&s, 16).unwrap());

    // Create parser to read an integer
    let int = text::int(10).map(|s: String| s.parse().unwrap()).padded();

    // Create colour parser
    let colour = hex.or(int).padded();

    let colour_parser = colour
        .clone()
        .delimited_by(just("#colour:"), just(','))
        .padded()
        .map(EmbedComponent::Colour);

    colour_parser
}
