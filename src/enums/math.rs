#[derive(Debug)]
/// An enum representing a VeaScript math expression.
pub enum MathExpr {
    /// A standard 64-bit float.
    Num(f64),

    /// A negative operator to flip the sign of a number.
    Neg(Box<Self>),

    /// An addition operator, to add two expressions together.
    Add(Box<Self>, Box<Self>),

    /// A subtraction operator, to subtract one expression from another..
    Sub(Box<Self>, Box<Self>),

    /// A multiplication operator, to multiply two expressions together.
    Mul(Box<Self>, Box<Self>),

    /// A division operator, to divide two expressions.
    Div(Box<Self>, Box<Self>),
}
