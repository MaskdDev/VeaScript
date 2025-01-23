#![allow(unused)]
use chumsky::prelude::*;
use parsers::math::{self, MathExpr};
use rand::seq::SliceRandom;
use rand::thread_rng;

// Import modules
mod parsers;
mod structs;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!(
        "{}",
        interpret_veascript(parse_veascript().parse(src).unwrap())
    )
}

#[derive(Debug)]
pub enum Expr {
    Math(MathExpr),
    Random(Vec<String>),
}

/// Parse a VeaScript expression.
pub fn parse_veascript() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
    // Try parsing random, then math
    parse_random().or(parse_math()).padded().repeated()
}

/// Parse a string of characters into a random expression.
pub fn parse_random() -> impl Parser<char, Expr, Error = Simple<char>> {
    // Create parser to read a string
    let string = just("\"")
        .ignore_then(filter(|c| *c != '\\' && *c != '\"').repeated())
        .then_ignore(just("\""))
        .padded()
        .collect::<String>();

    // Create parser to read random list
    let random = string
        .clone()
        .chain(just(',').ignore_then(string.clone()).repeated())
        .or_not()
        .flatten()
        .delimited_by(just("#random {"), just('}'))
        .map(Expr::Random)
        .labelled("random array");

    random.padded()
}

/// Parse a string of math characters into a math expression.
pub fn parse_math() -> impl Parser<char, Expr, Error = Simple<char>> {
    let math_parser = math::parse_math()
        .delimited_by(just("#math {"), just("}"))
        .map(Expr::Math)
        .labelled("math expression");

    math_parser.padded()
}

/// Interpret a vector of VeaScript expressions.
pub fn interpret_veascript(script: Vec<Expr>) -> String {
    // Create output
    let mut output = String::new();

    // Iterate over expressions
    for expression in script {
        match expression {
            Expr::Math(expression) => {
                output.push_str(&math::eval(&expression).to_string());
            }
            Expr::Random(options) => {
                output.push_str(options.choose(&mut thread_rng()).unwrap());
            }
        }
    }

    // Return output
    output
}
