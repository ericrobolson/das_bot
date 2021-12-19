use super::{Token, TokenizerErr, Tokens};

#[derive(PartialEq, Debug)]
pub enum Maker {
    Comment(String),
    Empty,
    Identifier(String),
    String(String),
}
impl Maker {
    /// Resets the state
    pub fn clear(&mut self) -> &mut Self {
        *self = Self::Empty;
        self
    }

    /// Begins a comment
    pub fn comment(self) -> Self {
        Self::Comment(String::default())
    }

    /// Begins an identifier
    pub fn identifier(mut self) -> Self {
        Self::Identifier(String::default())
    }

    /// Returns whether the state is a comment or not
    pub fn is_comment(&self) -> bool {
        match self {
            Self::Comment(_) => true,
            _ => false,
        }
    }

    /// Returns true when empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false,
        }
    }

    /// Returns true when an identifier
    pub fn is_identifier(&self) -> bool {
        match self {
            Self::Identifier(_) => true,
            _ => false,
        }
    }

    /// Returns whether it is creating a string or not
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_) => true,
            _ => false,
        }
    }

    /// Attempts to make a token.
    pub fn make_token(&mut self, tokens: &mut Tokens) -> Result<(), TokenizerErr> {
        match self {
            Self::Comment(comment) => {
                tokens.push(Token::Comment(comment.clone()));
            }
            Self::Empty => {}
            Self::Identifier(id) => tokens.push(Token::Identifier(id.clone())),
            Self::String(_) => return Err(TokenizerErr::UnclosedString),
        }

        self.clear();
        Ok(())
    }

    /// Creates a new instance
    pub fn new() -> Self {
        Self::Empty
    }

    /// Pushes the given character to the state
    pub fn push_char(&mut self, c: char) -> Result<(), TokenizerErr> {
        match self {
            Self::Comment(s) => {
                s.push(c);
                Ok(())
            }
            Self::Empty => Err(TokenizerErr::AddCharToUninitializedState),
            Self::Identifier(s) => {
                s.push(c);
                Ok(())
            }
            Self::String(s) => {
                s.push(c);
                Ok(())
            }
        }
    }

    /// Pushes the given string to the state
    pub fn push_str(&mut self, s: &str) -> Result<(), TokenizerErr> {
        for c in s.chars() {
            self.push_char(c)?;
        }

        Ok(())
    }

    /// Begins a string
    pub fn string(self) -> Self {
        Self::String(String::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod clear {
        use super::*;

        #[test]
        fn resets_to_empty() {
            assert_eq!(Maker::Empty, *Maker::new().comment().clear())
        }
    }

    mod comment {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let expected = Maker::Comment(String::default());
            assert_eq!(expected, Maker::new().comment())
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let expected = Maker::Identifier(String::default());
            assert_eq!(expected, Maker::new().identifier())
        }
    }

    mod is_comment {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().is_comment());
            assert_eq!(false, Maker::new().string().is_comment())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().comment().is_comment())
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().comment().is_empty());
            assert_eq!(false, Maker::new().string().is_empty())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().is_empty())
        }
    }

    mod is_identifier {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().comment().is_identifier());
            assert_eq!(false, Maker::new().string().is_identifier())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().identifier().is_identifier())
        }
    }

    mod is_string {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().comment().is_string());
            assert_eq!(false, Maker::new().is_string())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().string().is_string())
        }
    }

    mod make_token {
        use super::*;

        #[test]
        fn comment_adds_comment() {
            let expected: Tokens = vec![Token::Comment("Hello".into())];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().comment();
            maker.push_str("Hello").unwrap();
            let result = maker.make_token(&mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn empty_does_nothing() {
            let expected: Tokens = vec![];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new();

            let result = maker.make_token(&mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn identifier_adds_identifier() {
            let expected: Tokens = vec![Token::Identifier("Hello".into())];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().identifier();
            maker.push_str("Hello").unwrap();
            let result = maker.make_token(&mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn string_returns_err() {
            let expected: Tokens = vec![];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().string();
            maker.push_str("Hello").unwrap();
            let result = maker.make_token(&mut tokens);

            assert_eq!(Err(TokenizerErr::UnclosedString), result);
            assert_eq!(Maker::String("Hello".into()), maker);
            assert_eq!(expected, tokens);
        }
    }

    mod new {
        use super::*;

        #[test]
        fn returns_empty() {
            assert_eq!(Maker::Empty, Maker::new())
        }
    }

    mod push_char {
        use super::*;

        #[test]
        fn comment_is_added_to() {
            let mut maker = Maker::new().comment();

            let expected = Maker::Comment("c".into());
            let result = maker.push_char('c');

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn empty_returns_error() {
            let mut maker = Maker::new();

            let expected = Maker::new();
            let result = maker.push_char('c');

            assert_eq!(Err(TokenizerErr::AddCharToUninitializedState), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn identifier_is_added_to() {
            let mut maker = Maker::new().identifier();

            let expected = Maker::Identifier("c".into());
            let result = maker.push_char('c');

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn string_is_added_to() {
            let mut maker = Maker::new().string();

            let expected = Maker::String("c".into());
            let result = maker.push_char('c');

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }
    }

    mod push_str {
        use super::*;

        #[test]
        fn comment_is_added_to() {
            let mut maker = Maker::new().comment();

            let expected = Maker::Comment("cab".into());
            let result = maker.push_str("cab");

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn empty_returns_error() {
            let mut maker = Maker::new();

            let expected = Maker::new();
            let result = maker.push_str("cab");

            assert_eq!(Err(TokenizerErr::AddCharToUninitializedState), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn identifier_is_added_to() {
            let mut maker = Maker::new().identifier();

            let expected = Maker::Identifier("cab".into());
            let result = maker.push_str("cab");

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn string_is_added_to() {
            let mut maker = Maker::new().string();

            let expected = Maker::String("cab".into());
            let result = maker.push_str("cab");

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }
    }

    mod string {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let expected = Maker::String(String::default());
            assert_eq!(expected, Maker::new().string())
        }
    }
}
