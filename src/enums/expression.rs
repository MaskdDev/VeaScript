use crate::enums::{EmbedComponent, MathExpr};

/// A VeaScript expression.
#[derive(Debug)]
pub enum Expr {
    Text(String),
    Math(MathExpr),
    Random(Vec<String>),
    Embed(Vec<EmbedComponent>),
}
