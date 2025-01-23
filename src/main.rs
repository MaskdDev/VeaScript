#![allow(unused)]
use chumsky::chain::Chain;
use chumsky::prelude::*;
use enums::Expr;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serenity::all::Embed;
use structs::StoredEmbed;

// Import modules
mod builders;
mod enums;
mod helpers;
mod parsers;
mod runners;
mod structs;

fn main() {
    // Read .vs file
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    // Parse script
    match parse_veascript().parse(src) {
        Ok(script) => {
            // Build script
            match build_veascript(script) {
                Ok(output) => {
                    // Display output
                    println!(
                        "Content:\n{}\n\nEmbeds:\n{:?}",
                        output.content, output.embeds
                    )
                }
                Err(error) => {
                    println!("Build error: {}", error)
                }
            }
        }
        Err(error) => {
            println!("Parsing error: {:?}", error);
        }
    }
}

#[derive(Debug)]
pub struct VeaScriptOutput {
    /// The content produced by a VeaScript block.
    pub content: String,

    /// The embeds produced by a VeaScript block.
    pub embeds: Vec<StoredEmbed>,
}

impl VeaScriptOutput {
    /// Create a new output struct.
    pub fn new() -> Self {
        Self {
            content: String::new(),
            embeds: Vec::new(),
        }
    }

    /// Push to the output's content.
    pub fn push_content(&mut self, content: impl Into<String>) {
        self.content.push_str(&content.into());
    }

    /// Add an embed to the output.
    pub fn push_embed(&mut self, embed: StoredEmbed) {
        self.embeds.push(embed);
    }
}

/// Parse a VeaScript expression.
pub fn parse_veascript() -> impl Parser<char, Vec<Expr>, Error = Simple<char>> {
    // Try parsing random, then math
    parsers::random()
        .or(parsers::math())
        .or(parsers::text())
        .or(parsers::embed())
        .repeated()
        .then_ignore(end())
}

/// Build VeaScript output from a vector of VeaScript expressions.
pub fn build_veascript(script: Vec<Expr>) -> Result<VeaScriptOutput, String> {
    // Create output
    let mut output = VeaScriptOutput::new();

    // Iterate over expressions
    for expression in script {
        match expression {
            Expr::Math(expression) => {
                output.push_content(&runners::eval(&expression).to_string());
            }
            Expr::Random(options) => {
                output.push_content(options.choose(&mut thread_rng()).unwrap());
            }
            Expr::Text(text) => {
                output.push_content(&text);
            }
            Expr::Embed(components) => {
                output.push_embed(builders::embed(components)?);
            }
        }
    }

    // Return output
    Ok(output)
}
