use crate::enums::EmbedComponent;
use crate::helpers::hexadecimal;
use crate::structs::StoredEmbed;
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
                }
            }
        }
    }

    // Return embed
    Ok(embed)
}
