/// A VeaScript embed component.
#[derive(Debug)]
pub enum EmbedComponent {
    Title(String),
    Description(String),
    Colour(i32),
    Image(String),
    Thumbnail(String),
    Url(String),
}

/// A VeaScript embed author component.
#[derive(Debug)]
pub enum EmbedAuthorComponent {
    Name(String),
    Url(String),
    IconUrl(String),
}

/// A VeaScript embed footer component.
#[derive(Debug)]
pub enum EmbedFooterComponent {
    Text(String),
    Timestamp(u64),
    IconUrl(String),
}

/// A VeaScript embed field component.
#[derive(Debug)]
pub enum EmbedFieldComponent {
    Name(String),
    Value(String),
    Inline(bool),
}
