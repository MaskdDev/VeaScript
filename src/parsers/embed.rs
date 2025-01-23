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
        .or(parse_embed_colour())
        .or(parse_embed_image())
        .or(parse_embed_thumbnail())
        .or(parse_embed_url());

    // Return embed component parser
    component_parser.padded()
}

/// Parse a string field.
pub fn parse_string_field(
    tag: &'static str,
    component: impl Fn(String) -> EmbedComponent,
) -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Return title parser
    parsers::string()
        .delimited_by(just(tag), just(','))
        .map(component)
}

/// Parse an embed title.
pub fn parse_embed_title() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parse_string_field("#title:", EmbedComponent::Title)
}

/// Parse an embed description.
pub fn parse_embed_description() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parse_string_field("#description:", EmbedComponent::Description)
}

/// Parse an embed image.
pub fn parse_embed_image() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parse_string_field("#image:", EmbedComponent::Image)
}

/// Parse an embed thumbnail.
pub fn parse_embed_thumbnail() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parse_string_field("#thumbnail:", EmbedComponent::Thumbnail)
}

/// Parse an embed url.
pub fn parse_embed_url() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parse_string_field("#url:", EmbedComponent::Url)
}

/// Parse an embed colour
pub fn parse_embed_colour() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Create colour parser
    let colour = parsers::hex().or(parsers::int32()).padded();

    // Return colour field parser
    colour
        .delimited_by(just("#colour:"), just(','))
        .padded()
        .map(EmbedComponent::Colour)
}
