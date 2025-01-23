use chumsky::prelude::*;

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

/// Parse a string of characters into a math expression.
pub fn parse_math() -> impl Parser<char, MathExpr, Error = Simple<char>> {
    // Create recursive parser
    recursive(|expr| {
        // Create parser to read in an unsigned integer
        let int = text::int(10)
            .map(|s: String| MathExpr::Num(s.parse().unwrap()))
            .padded();

        // Create atom parser, which is either an unsigned integer or paratheses containing an expression
        let atom = int.or(expr.delimited_by(just('('), just(')'))).padded();

        // Create operator parser
        let op = |c| just(c).padded();

        // Create unary parser, which is an atom with an optional '-' in front of it.
        let unary = op('-')
            .repeated()
            .then(atom)
            .foldr(|_op, rhs| MathExpr::Neg(Box::new(rhs)));

        // Create product parser, which parses multiplication and division.
        let product = unary
            .clone()
            .then(
                op('*')
                    .to(MathExpr::Mul as fn(_, _) -> _)
                    .or(op('/').to(MathExpr::Div as fn(_, _) -> _))
                    .then(unary)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        // Create sum parser, which parses addition and subtraction after multiplication and division.
        let sum = product
            .clone()
            .then(
                op('+')
                    .to(MathExpr::Add as fn(_, _) -> _)
                    .or(op('-').to(MathExpr::Sub as fn(_, _) -> _))
                    .then(product)
                    .repeated(),
            )
            .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

        // Return sum parser.
        sum
    })
}

/// Evaluate a math expression.
pub fn eval(expression: &MathExpr) -> f64 {
    // Match expression type
    match expression {
        MathExpr::Num(x) => *x,
        MathExpr::Neg(a) => -eval(a),
        MathExpr::Add(a, b) => eval(a) + eval(b),
        MathExpr::Sub(a, b) => eval(a) - eval(b),
        MathExpr::Mul(a, b) => eval(a) * eval(b),
        MathExpr::Div(a, b) => eval(a) / eval(b),
    }
}
