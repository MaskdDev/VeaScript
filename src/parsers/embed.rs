use crate::enums::{EmbedAuthorComponent, EmbedComponent, EmbedFooterComponent, Expr};
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
        .or(parse_embed_url())
        .or(parse_embed_author())
        .or(parse_embed_footer())
        .or(parse_embed_timestamp());

    // Return embed component parser
    component_parser.padded()
}

/// Parse a string field for an embed.
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

/// Parse an embed timestamp.
pub fn parse_embed_timestamp() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    parsers::int64()
        .delimited_by(just("#timestamp:"), just(','))
        .map(EmbedComponent::Timestamp)
}

/// Parse an embed colour.
pub fn parse_embed_colour() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Create colour parser
    let colour = parsers::hex().or(parsers::int32()).padded();

    // Return colour field parser
    colour
        .delimited_by(just("#colour:"), just(','))
        .padded()
        .map(EmbedComponent::Colour)
}

/// Parse an embed author into a VeaScript embed component.
pub fn parse_embed_author() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Return embed parser
    parse_embed_author_raw().map(EmbedComponent::Author)
}

/// Parse an embed author into a vector of embed components.
pub fn parse_embed_author_raw() -> impl Parser<char, Vec<EmbedAuthorComponent>, Error = Simple<char>>
{
    // Create author parser
    let author_parser = parse_embed_author_component()
        .repeated()
        .delimited_by(just("#author {"), just("}"))
        .collect();

    // Return author parser
    author_parser.padded()
}

/// Parse an embed author component.
pub fn parse_embed_author_component(
) -> impl Parser<char, EmbedAuthorComponent, Error = Simple<char>> {
    // Create author component parser
    let component_parser = parse_embed_author_name()
        .or(parse_embed_author_url())
        .or(parse_embed_author_icon_url());

    // Return author component parser
    component_parser.padded()
}

/// Parse a string field for an embed author.
pub fn parse_author_string_field(
    tag: &'static str,
    component: impl Fn(String) -> EmbedAuthorComponent,
) -> impl Parser<char, EmbedAuthorComponent, Error = Simple<char>> {
    // Return title parser
    parsers::string()
        .delimited_by(just(tag), just(','))
        .map(component)
}

/// Parse an embed author's name.
pub fn parse_embed_author_name() -> impl Parser<char, EmbedAuthorComponent, Error = Simple<char>> {
    parse_author_string_field("#name:", EmbedAuthorComponent::Name)
}

/// Parse an embed author's url.
pub fn parse_embed_author_url() -> impl Parser<char, EmbedAuthorComponent, Error = Simple<char>> {
    parse_author_string_field("#url:", EmbedAuthorComponent::Url)
}

/// Parse an embed author's icon url.
pub fn parse_embed_author_icon_url() -> impl Parser<char, EmbedAuthorComponent, Error = Simple<char>>
{
    parse_author_string_field("#icon_url:", EmbedAuthorComponent::IconUrl)
}

/// Parse an embed footer into a VeaScript embed component.
pub fn parse_embed_footer() -> impl Parser<char, EmbedComponent, Error = Simple<char>> {
    // Return embed parser
    parse_embed_footer_raw().map(EmbedComponent::Footer)
}

/// Parse an embed footer into a vector of embed components.
pub fn parse_embed_footer_raw() -> impl Parser<char, Vec<EmbedFooterComponent>, Error = Simple<char>>
{
    // Create footer parser
    let footer_parser = parse_embed_footer_component()
        .repeated()
        .delimited_by(just("#footer {"), just("}"))
        .collect();

    // Return footer parser
    footer_parser.padded()
}

/// Parse an embed footer component.
pub fn parse_embed_footer_component(
) -> impl Parser<char, EmbedFooterComponent, Error = Simple<char>> {
    // Create footer component parser
    let component_parser = parse_embed_footer_text().or(parse_embed_footer_icon_url());

    // Return footer component parser
    component_parser.padded()
}

/// Parse a string field for an embed footer.
pub fn parse_footer_string_field(
    tag: &'static str,
    component: impl Fn(String) -> EmbedFooterComponent,
) -> impl Parser<char, EmbedFooterComponent, Error = Simple<char>> {
    // Return title parser
    parsers::string()
        .delimited_by(just(tag), just(','))
        .map(component)
}

/// Parse an embed footer's text.
pub fn parse_embed_footer_text() -> impl Parser<char, EmbedFooterComponent, Error = Simple<char>> {
    parse_footer_string_field("#text:", EmbedFooterComponent::Text)
}

/// Parse an embed footer's icon url.
pub fn parse_embed_footer_icon_url() -> impl Parser<char, EmbedFooterComponent, Error = Simple<char>>
{
    parse_footer_string_field("#icon_url:", EmbedFooterComponent::IconUrl)
}
