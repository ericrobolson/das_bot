mod consts;
mod maker;

use consts::*;
use maker::*;

pub type Tokens = Vec<(Location, Token)>;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Comment(String),
    Identifier(String),
    String(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Location {
    pub column: usize,
    pub file: Option<String>,
    pub line: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenizerErr {
    AddCharToUninitializedState { char: char, location: Location },
    UnclosedString { string: String, location: Location },
}

pub fn execute(input: &String, file: Option<String>) -> Result<Tokens, TokenizerErr> {
    let mut tokens = vec![];
    let mut maker = Maker::new();

    let mut location = Location {
        column: 0,
        file,
        line: 0,
    };

    // sanitize input, such as simplifying newlines
    let input = input.replace('\r', "");

    for c in input.chars() {
        println!("{}", c);
        if !maker.is_comment() && c == COMMENT {
            maker.make_token(&location, &mut tokens)?;
            maker = maker.comment(location.clone());
        } else if c == NEWLINE && maker.is_comment() {
            maker.make_token(&location, &mut tokens)?;
        } else if c != NEWLINE && !maker.is_empty() {
            maker.push_char(c, &location)?;
        }

        // Increment things
        location.column += 1;
        if c == NEWLINE {
            location.column = 0;
            location.line += 1;
        }
    }

    // Drain the WIP token
    maker.make_token(&location, &mut tokens)?;

    Ok(tokens)
}

fn terminal_char(c: char) -> bool {
    match c {
        NEWLINE | ' ' | '\t' | COMMENT => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_terminal_char {
        use super::*;

        #[test]
        fn newline_returns_true() {
            assert_eq!(true, terminal_char('\n'));
        }

        #[test]
        fn space_returns_true() {
            assert_eq!(true, terminal_char(' '));
        }

        #[test]
        fn tab_returns_true() {
            assert_eq!(true, terminal_char('\t'));
        }

        #[test]
        fn comment_returns_true() {
            assert_eq!(true, terminal_char(COMMENT));
        }
    }
}
