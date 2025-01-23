use serde::{Deserialize, Serialize};
use serenity::all::{Colour, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, Timestamp};

#[derive(Serialize, Deserialize, Debug)]
/// A stored embed that can be used by the bot.
pub struct StoredEmbed {
    /// The author for the embed.
    pub author: Option<StoredEmbedAuthor>,

    /// The embed's colour
    pub colour: i32,

    /// The title of the embed.
    pub title: Option<String>,

    /// The url for the embed.
    pub url: Option<String>,

    /// The description of the embed.
    pub description: Option<String>,

    /// The fields of the embed
    pub fields: Vec<StoredEmbedField>,

    /// The image url for the embed.
    pub image_url: Option<String>,

    /// The thumbnail url for the embed.
    pub thumbnail_url: Option<String>,

    /// The footer for the embed.
    pub footer: Option<StoredEmbedFooter>,

    /// The timestamp for the embed.
    pub timestamp: Option<i64>,
}

impl From<StoredEmbed> for CreateEmbed {
    fn from(value: StoredEmbed) -> Self {
        // Create base embed
        let mut embed = CreateEmbed::new().colour(value.colour);

        // Add embed title if needed
        if let Some(title) = value.title {
            embed = embed.title(title);
        }

        // Add embed author if needed
        if let Some(author) = value.author {
            embed = embed.author(author.into());
        }

        // Add embed url if needed
        if let Some(url) = value.url {
            embed = embed.url(url);
        }

        // Add embed description if needed
        if let Some(description) = value.description {
            embed = embed.description(description);
        }

        // Add embed fields
        for field in value.fields {
            embed = embed.field(field.name, field.value, field.inline)
        }

        // Add embed image if needed
        if let Some(image_url) = value.image_url {
            embed = embed.image(image_url);
        }

        // Add embed thumbnail if needed
        if let Some(thumbnail_url) = value.thumbnail_url {
            embed = embed.thumbnail(thumbnail_url);
        }

        // Add embed footer if needed
        if let Some(footer) = value.footer {
            embed = embed.footer(footer.into())
        }

        // Add embed timestamp if needed
        if let Some(timestamp) = value.timestamp {
            embed = embed.timestamp(
                Timestamp::from_unix_timestamp(timestamp)
                    .expect("Stored embed should always contain a valid timestamp"),
            )
        }

        // Return embed
        embed
    }
}

impl StoredEmbed {
    /// Create a new stored embed.
    pub fn new() -> Self {
        Self {
            author: None,
            colour: 0,
            title: None,
            url: None,
            description: None,
            fields: Vec::new(),
            image_url: None,
            thumbnail_url: None,
            footer: None,
            timestamp: None,
        }
    }

    /// Set the embed's author.
    pub fn author(mut self, author: StoredEmbedAuthor) -> Self {
        self.author = Some(author);
        self
    }

    /// Set the embed's colour.
    pub fn colour(mut self, colour: impl Into<Colour>) -> Self {
        self.colour = colour.into().0 as i32;
        self
    }

    /// Set the embed's title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set the embed's url.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the embed's description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Add a field to the embed.
    pub fn field(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
        inline: bool,
    ) -> Self {
        self.fields.push(StoredEmbedField {
            name: name.into(),
            value: value.into(),
            inline,
        });
        self
    }

    /// Set the embed's image.
    pub fn image(mut self, image_url: impl Into<String>) -> Self {
        self.image_url = Some(image_url.into());
        self
    }

    /// Set the embed's thumbnail.
    pub fn thumbnail(mut self, thumbnail_url: impl Into<String>) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    /// Set the embed's footer.
    pub fn footer(mut self, footer: StoredEmbedFooter) -> Self {
        self.footer = Some(footer);
        self
    }

    /// Set the embed's timestamp.
    pub fn timestamp(mut self, timestamp: i64) -> Self {
        self.timestamp = Some(timestamp);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// A stored embed author that can be used by the bot.
pub struct StoredEmbedAuthor {
    /// The embed author's name.
    pub name: String,

    /// The url for the author.
    pub url: Option<String>,

    /// The icon url for the author.
    pub icon_url: Option<String>,
}

impl From<StoredEmbedAuthor> for CreateEmbedAuthor {
    fn from(value: StoredEmbedAuthor) -> Self {
        // Create base author
        let mut author = CreateEmbedAuthor::new(value.name);

        // Add url if present
        if let Some(url) = value.url {
            author = author.url(url)
        }

        // Add icon url if present
        if let Some(icon_url) = value.icon_url {
            author = author.icon_url(icon_url)
        }

        // Return author
        author
    }
}

impl StoredEmbedAuthor {
    /// Create a new embed author using a given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            url: None,
            icon_url: None,
        }
    }

    /// Set the embed author's name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the embed author's url.
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the embed author's icon url.
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// A stored embed footer that can be used by the bot.
pub struct StoredEmbedFooter {
    /// The embed footer's text.
    pub text: String,

    /// The icon url for the footer.
    pub icon_url: Option<String>,
}

impl StoredEmbedFooter {
    /// Create a new embed footer using a given piece of text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            icon_url: None,
        }
    }

    /// Set the embed footer's text.
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    /// Set the embed footer's icon url.
    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.icon_url = Some(icon_url.into());
        self
    }
}

impl From<StoredEmbedFooter> for CreateEmbedFooter {
    fn from(value: StoredEmbedFooter) -> Self {
        // Create base footer
        let mut footer = CreateEmbedFooter::new(value.text);

        // Add icon url if present
        if let Some(icon_url) = value.icon_url {
            footer = footer.icon_url(icon_url)
        }

        // Return footer
        footer
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// A stored embed field that can be used by the bot.
pub struct StoredEmbedField {
    /// The name of the field.
    pub name: String,

    /// The value of the field.
    pub value: String,

    /// Whether or not the field is inline.
    pub inline: bool,
}
