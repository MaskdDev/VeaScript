/// A VeaScript embed component.
#[derive(Debug)]
pub enum EmbedComponent {
    Title(String),
    Description(String),
    Colour(i32),
}
