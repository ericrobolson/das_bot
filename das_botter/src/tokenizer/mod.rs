mod consts;
mod maker;

use consts::*;
use maker::*;

pub type Tokens = Vec<Token>;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Identifier(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenizerErr {
    AddCharToUninitializedState,
    UnclosedString,
}

pub fn execute(input: &String) -> Result<Tokens, TokenizerErr> {
    let mut tokens = vec![];
    let mut maker = Maker::new();
    let mut line = 0;
    let mut column = 0;

    // sanitize input, such as simplifying newlines
    let input = input.replace('\r', "");

    for c in input.chars() {
        println!("{}", c);
        if !maker.is_comment() && c == COMMENT {
            maker.make_token(&mut tokens)?;
            maker = maker.comment();
        } else if c == NEWLINE && maker.is_comment() {
            maker.make_token(&mut tokens)?;
        } else if c != NEWLINE && !maker.is_empty() {
            maker.push_char(c)?;
        }

        // Increment things
        column += 1;
        if c == NEWLINE {
            column = 0;
            line += 1;
        }
    }

    // Drain the WIP token
    maker.make_token(&mut tokens)?;

    Ok(tokens)
}

fn terminal_char(c: char) -> bool {
    match c {
        NEWLINE | ' ' | '\t' | COMMENT => true,
        _ => false,
    }
}
