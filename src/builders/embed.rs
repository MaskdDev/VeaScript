use crate::enums::{
    EmbedAuthorComponent, EmbedComponent, EmbedFieldComponent, EmbedFooterComponent,
};
use crate::helpers::{hexadecimal, validation};
use crate::structs::{StoredEmbed, StoredEmbedAuthor, StoredEmbedField, StoredEmbedFooter};
use chumsky::prelude::*;

/// Build a stored embed struct using a vector of VeaScript embed components.
pub fn build_embed(components: Vec<EmbedComponent>) -> Result<StoredEmbed, String> {
    // Create new stored embed
    let mut embed = StoredEmbed::new();

    // Create colour set flag
    let mut colour_set = false;

    // Iterate over components
    for component in components {
        match component {
            EmbedComponent::Author(components) => {
                // Add author to embed
                embed = embed.author(build_author(components)?)
            }
            EmbedComponent::Title(title) => {
                // Get title length
                let title_length = title.len();

                // Check if title has already been set
                if embed.title.is_some() {
                    // Return multiple title error
                    return Err(String::from("You can only have one title for an embed."));
                } else if title_length > 256 {
                    // Return length error
                    return Err(format!(
                        "The length of your title ({}) is above the maximum of 256 characters.",
                        title_length
                    ));
                } else {
                    // Add title to embed
                    embed = embed.title(title);
                }
            }
            EmbedComponent::Description(description) => {
                // Get description length
                let description_length = description.len();

                // Check if description has already been set
                if embed.description.is_some() {
                    // Return multiple description error
                    return Err(String::from(
                        "You can only have one description for an embed.",
                    ));
                } else if description_length > 4096 {
                    // Return length error
                    return Err(format!(
                        "The length of your title ({}) is above the maximum of 4096 characters.",
                        description_length
                    ));
                } else {
                    // Add description to embed
                    embed = embed.description(description);
                }
            }
            EmbedComponent::Colour(colour) => {
                // Check if colour has already been set
                if colour_set {
                    // Return multiple colour error
                    return Err(String::from(
                        "You can only set the colour for an embed once.",
                    ));
                } else if colour > 0xFFFFFF {
                    // Return length error
                    return Err(format!(
                        "Your colour value (#{}) is greater than the limit of #FFFFFF.",
                        hexadecimal::hex_to_str(colour)
                    ));
                } else {
                    // Set embed colour
                    embed = embed.colour(colour);

                    // Set colour set flag
                    colour_set = true;
                }
            }
            EmbedComponent::Fields(fields) => {
                // Build embed fields
                let mut fields = build_fields(fields)?;

                // Check if field limit has been exceeded.
                if embed.fields.len() + fields.len() > 25 {
                    // Return too many fields error
                    return Err(String::from(
                        "An embed can only have a maximum of 25 fields.",
                    ));
                }

                // Add new fields to embed
                embed.fields.append(&mut fields);
            }
            EmbedComponent::Image(image_url) => {
                // Check if the image url is valid.
                validation::url(&image_url, "Image URL")?;

                // Check if image has already been set
                if embed.image_url.is_some() {
                    // Return multiple image error
                    return Err(String::from("You can only have one image for an embed."));
                } else {
                    // Set embed image
                    embed = embed.image(image_url);
                }
            }
            EmbedComponent::Thumbnail(thumbnail_url) => {
                // Check if the thumbnail url is valid.
                validation::url(&thumbnail_url, "Thumbnail URL")?;

                // Check if image has already been set
                if embed.thumbnail_url.is_some() {
                    // Return multiple thumbnail error
                    return Err(String::from(
                        "You can only have one thumbnail for an embed.",
                    ));
                } else {
                    // Set embed thumbnail
                    embed = embed.thumbnail(thumbnail_url);
                }
            }
            EmbedComponent::Url(embed_url) => {
                // Check if the embed url is valid.
                validation::url(&embed_url, "Embed URL")?;

                // Check if url has already been set
                if embed.url.is_some() {
                    // Return multiple url error
                    return Err(String::from("You can only have one url for an embed."));
                } else {
                    // Set embed url
                    embed = embed.url(embed_url);
                }
            }
            EmbedComponent::Footer(components) => {
                // Add footer to embed
                embed = embed.footer(build_footer(components)?);
            }
            EmbedComponent::Timestamp(stamp) => {
                // Check if timestamp is not negative
                if stamp >= 0 {
                    // Add timestamp to embed
                    embed = embed.timestamp(stamp);
                } else {
                    // Return invalid timestamp error
                    return Err(format!("Invalid embed timestamp ({}) - the timestamp for an embed must be greater than 0.", stamp));
                }
            }
        }
    }

    // Get the total number of characters in the embed.
    let total_chars = embed.total_chars();

    // Check if embed characters doesn't exceed 6000.
    if total_chars > 6000 {
        // Return total chars error
        Err(format!(
            "Your embed has {} characters in total. An embed cannot exceed 6000 total characters.",
            total_chars
        ))
    } else {
        // Return embed
        Ok(embed)
    }
}

// Build a stored embed author struct using a vector of VeaScript embed author components.
pub fn build_author(components: Vec<EmbedAuthorComponent>) -> Result<StoredEmbedAuthor, String> {
    // Create new stored embed author
    let mut author = StoredEmbedAuthor::new("");

    // Create name set flag
    let mut name_set = false;

    // Iterate over components
    for component in components {
        match component {
            EmbedAuthorComponent::Name(name) => {
                // Get author name length
                let name_length = name.len();

                // Check if author name has already been set
                if name_set {
                    // Return multiple author name error
                    return Err(String::from(
                        "You can only have one author name for an embed.",
                    ));
                } else if name_length > 256 {
                    // Return length error
                    return Err(format!(
                        "The length of your author name ({}) is above the maximum of 256 characters.",
                        name_length
                    ));
                } else {
                    // Add name to embed author
                    author = author.name(name);

                    // Set name set flag
                    name_set = true;
                }
            }
            EmbedAuthorComponent::Url(author_url) => {
                // Check if the author url is valid.
                validation::url(&author_url, "Embed Author URL")?;

                // Check if url has already been set
                if author.url.is_some() {
                    // Return multiple url error
                    return Err(String::from(
                        "You can only have one url for an embed author.",
                    ));
                } else {
                    // Set embed author url
                    author = author.url(author_url);
                }
            }
            EmbedAuthorComponent::IconUrl(icon_url) => {
                // Check if the author icon url is valid.
                validation::url(&icon_url, "Embed Author Icon URL")?;

                // Check if url has already been set
                if author.icon_url.is_some() {
                    // Return multiple icon url error
                    return Err(String::from(
                        "You can only have one icon url for an embed author.",
                    ));
                } else {
                    // Set embed icon author url
                    author = author.icon_url(icon_url);
                }
            }
        }
    }

    // Check if embed author has some field set
    if author.name.is_empty() && author.url.is_none() && author.icon_url.is_none() {
        // Return empty author error
        return Err(String::from("An embed cannot have an empty author."));
    }

    // Return embed author
    Ok(author)
}

// Build a stored embed footer struct using a vector of VeaScript embed footer components.
pub fn build_footer(components: Vec<EmbedFooterComponent>) -> Result<StoredEmbedFooter, String> {
    // Create new stored embed footer
    let mut footer = StoredEmbedFooter::new("");

    // Create text set flag
    let mut text_set = false;

    // Iterate over components
    for component in components {
        match component {
            EmbedFooterComponent::Text(text) => {
                // Get footer text length
                let text_length = text.len();

                // Check if footer text has already been set
                if text_set {
                    // Return multiple footer text error
                    return Err(String::from(
                        "You can only have one footer text for an embed.",
                    ));
                } else if text_length > 2048 {
                    // Return length error
                    return Err(format!(
                        "The length of your footer text ({}) is above the maximum of 2048 characters.",
                        text_length
                    ));
                } else {
                    // Add text to embed footer
                    footer = footer.text(text);

                    // Set text set flag
                    text_set = true;
                }
            }
            EmbedFooterComponent::IconUrl(icon_url) => {
                // Check if the footer icon url is valid.
                validation::url(&icon_url, "Embed Footer Icon URL")?;

                // Check if url has already been set
                if footer.icon_url.is_some() {
                    // Return multiple icon url error
                    return Err(String::from(
                        "You can only have one icon url for an embed footer.",
                    ));
                } else {
                    // Set embed icon footer url
                    footer = footer.icon_url(icon_url);
                }
            }
        }
    }

    // Check if embed footer has some field set
    if footer.text.is_empty() && footer.icon_url.is_none() {
        // Return empty footer error
        return Err(String::from("An embed cannot have an empty footer."));
    }

    // Return embed footer
    Ok(footer)
}

// Build a vector of stored embed field structs using a vector of vectors of VeaScript embed field components.
pub fn build_fields(
    fields: Vec<Vec<EmbedFieldComponent>>,
) -> Result<Vec<StoredEmbedField>, String> {
    println!("{:?}", fields);

    // Create built fields vector
    let mut built_fields: Vec<StoredEmbedField> = Vec::new();

    // Iterate over fields
    for field_components in fields {
        // Create field
        let mut field = StoredEmbedField::new();

        // Create name set, value set and inline set flags
        let mut name_set = false;
        let mut value_set = false;
        let mut inline_set = false;

        // Iterate over field components
        for component in field_components {
            match component {
                EmbedFieldComponent::Name(name) => {
                    // Get field name length
                    let name_length = name.len();

                    // Check if field name has already been set
                    if name_set {
                        // Return multiple field name error
                        return Err(String::from(
                            "You can only have one field name per embed field.",
                        ));
                    } else if name_length > 256 {
                        // Return length error
                        return Err(format!(
                        "The length of one of your field names ({}) is above the maximum of 256 characters.",
                        name_length
                    ));
                    } else {
                        // Add name to embed field
                        field = field.name(name);

                        // Set name set flag
                        name_set = true;
                    }
                }
                EmbedFieldComponent::Value(value) => {
                    // Get field value length
                    let value_length = value.len();

                    // Check if field value has already been set
                    if value_set {
                        // Return multiple field value error
                        return Err(String::from(
                            "You can only have one field value per embed field.",
                        ));
                    } else if value_length > 1024 {
                        // Return length error
                        return Err(format!(
                        "The length of one of your field values ({}) is above the maximum of 1024 characters.",
                        value_length
                    ));
                    } else {
                        // Add value to embed field
                        field = field.value(value);

                        // Set value set flag
                        value_set = true;
                    }
                }
                EmbedFieldComponent::Inline(inline) => {
                    // Check if field value has already been set
                    if inline_set {
                        // Return multiple field value error
                        return Err(String::from(
                            "You can only set inline once per embed field.",
                        ));
                    } else {
                        // Add inline to embed field
                        field = field.inline(inline);

                        // Set inline set flag
                        inline_set = true;
                    }
                }
            }
        }

        // Check if embed field has name and value set
        if field.name.is_empty() || field.value.is_empty() {
            // Return not set field error
            return Err(String::from(
                "All of your embed fields must have both a name and a value.",
            ));
        }

        // Add field to built fields
        built_fields.push(field);
    }

    // Return embed fields
    Ok(built_fields)
}
