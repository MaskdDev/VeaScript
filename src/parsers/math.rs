use crate::enums::{Expr, MathExpr};
use chumsky::prelude::*;

/// Parse a string of math characters into a VeaScript math expression.
pub fn parse_math() -> impl Parser<char, Expr, Error = Simple<char>> {
    let math_parser = parse_math_raw()
        .delimited_by(just("#math {"), just("}"))
        .map(Expr::Math)
        .labelled("math expression");

    math_parser.padded()
}

/// Parse a string of characters into a math expression.
pub fn parse_math_raw() -> impl Parser<char, MathExpr, Error = Simple<char>> {
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
