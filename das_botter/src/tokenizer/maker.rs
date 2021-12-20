use super::{Location, Token, TokenizerErr, Tokens};

#[derive(PartialEq, Debug)]
pub enum Maker {
    Comment { location: Location, string: String },
    Empty,
    Identifier { location: Location, string: String },
    String { location: Location, string: String },
}
impl Maker {
    /// Resets the state
    pub fn clear(&mut self) -> &mut Self {
        *self = Self::Empty;
        self
    }

    /// Begins a comment
    pub fn comment(self, location: Location) -> Self {
        Self::Comment {
            location,
            string: String::default(),
        }
    }

    /// Begins an identifier
    pub fn identifier(self, location: Location) -> Self {
        Self::Identifier {
            location,
            string: String::default(),
        }
    }

    /// Returns whether the state is a comment or not
    pub fn is_comment(&self) -> bool {
        match self {
            Self::Comment { .. } => true,
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
            Self::Identifier { .. } => true,
            _ => false,
        }
    }

    /// Returns whether it is creating a string or not
    pub fn is_string(&self) -> bool {
        match self {
            Self::String { .. } => true,
            _ => false,
        }
    }

    /// Attempts to make a string, if not a string simply attempts to make the token.
    pub fn make_string(
        &mut self,
        current_location: &Location,
        tokens: &mut Tokens,
    ) -> Result<(), TokenizerErr> {
        if let Self::String { location, string } = self {
            tokens.push((location.clone(), Token::String(string.clone())));
            self.clear();
            Ok(())
        } else {
            self.make_token(current_location, tokens)
        }
    }

    /// Attempts to make a token.
    pub fn make_token(
        &mut self,
        current_location: &Location,
        tokens: &mut Tokens,
    ) -> Result<(), TokenizerErr> {
        match self {
            Self::Comment { location, string } => {
                tokens.push((location.clone(), Token::Comment(string.clone())));
            }
            Self::Empty => {}
            Self::Identifier { location, string } => {
                tokens.push((location.clone(), Token::Identifier(string.clone())))
            }
            Self::String { string, .. } => {
                return Err(TokenizerErr::UnclosedString {
                    string: string.clone(),
                    location: current_location.clone(),
                })
            }
        }

        self.clear();
        Ok(())
    }

    /// Creates a new instance
    pub fn new() -> Self {
        Self::Empty
    }

    /// Pushes the given character to the state
    pub fn push_char(&mut self, c: char, current_location: &Location) -> Result<(), TokenizerErr> {
        match self {
            Self::Comment { string, .. } => {
                string.push(c);
                Ok(())
            }
            Self::Empty => Err(TokenizerErr::AddCharToUninitializedState {
                char: c,
                location: current_location.clone(),
            }),
            Self::Identifier { string, .. } => {
                string.push(c);
                Ok(())
            }
            Self::String { string, .. } => {
                string.push(c);
                Ok(())
            }
        }
    }

    /// Pushes the given string to the state
    pub fn push_str(&mut self, s: &str, current_location: &Location) -> Result<(), TokenizerErr> {
        for c in s.chars() {
            self.push_char(c, &current_location)?;
        }

        Ok(())
    }

    /// Begins a string
    pub fn string(self, location: Location) -> Self {
        Self::String {
            location,
            string: String::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loc() -> Location {
        Location {
            column: 9,
            line: 299,
            file: None,
        }
    }

    fn loc2() -> Location {
        Location {
            column: 93,
            line: 2992,
            file: Some("wut".into()),
        }
    }

    mod clear {
        use super::*;

        #[test]
        fn resets_to_empty() {
            assert_eq!(
                Maker::Empty,
                *Maker::new().comment(Location::default()).clear()
            )
        }
    }

    mod comment {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let location = Location {
                column: 9,
                line: 299,
                file: None,
            };

            let expected = Maker::Comment {
                location: location.clone(),
                string: String::default(),
            };
            assert_eq!(expected, Maker::new().comment(location))
        }
    }

    mod identifier {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let expected = Maker::Identifier {
                location: loc(),
                string: String::default(),
            };
            assert_eq!(expected, Maker::new().identifier(loc()))
        }
    }

    mod is_comment {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().is_comment());
            assert_eq!(false, Maker::new().string(loc()).is_comment())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().comment(Location::default()).is_comment())
        }
    }

    mod is_empty {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().comment(Location::default()).is_empty());
            assert_eq!(false, Maker::new().string(loc()).is_empty())
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
            assert_eq!(
                false,
                Maker::new().comment(Location::default()).is_identifier()
            );
            assert_eq!(false, Maker::new().string(loc()).is_identifier())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().identifier(loc()).is_identifier())
        }
    }

    mod is_string {
        use super::*;

        #[test]
        fn false_case() {
            assert_eq!(false, Maker::new().comment(Location::default()).is_string());
            assert_eq!(false, Maker::new().is_string())
        }

        #[test]
        fn true_case() {
            assert_eq!(true, Maker::new().string(loc()).is_string())
        }
    }

    mod make_string {
        use super::*;

        #[test]
        fn string_makes_string() {
            let expected: Tokens = vec![(loc(), Token::String("Hello".into()))];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().string(loc());
            maker.push_str("Hello", &loc2()).unwrap();
            let result = maker.make_string(&loc2(), &mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn not_string_does_normal_make_token() {
            let expected: Tokens = vec![(Location::default(), Token::Comment("Hello".into()))];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().comment(Location::default());
            maker.push_str("Hello", &loc2()).unwrap();
            let result = maker.make_string(&loc2(), &mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }
    }

    mod make_token {
        use super::*;

        #[test]
        fn comment_adds_comment() {
            let expected: Tokens = vec![(Location::default(), Token::Comment("Hello".into()))];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().comment(Location::default());
            maker.push_str("Hello", &loc2()).unwrap();
            let result = maker.make_token(&loc2(), &mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn empty_does_nothing() {
            let expected: Tokens = vec![];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new();

            let result = maker.make_token(&loc2(), &mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn identifier_adds_identifier() {
            let expected: Tokens = vec![(loc(), Token::Identifier("Hello".into()))];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().identifier(loc());
            maker.push_str("Hello", &loc2()).unwrap();
            let result = maker.make_token(&loc2(), &mut tokens);

            assert_eq!(Ok(()), result);
            assert_eq!(Maker::Empty, maker);
            assert_eq!(expected, tokens);
        }

        #[test]
        fn string_returns_err() {
            let expected: Tokens = vec![];
            let mut tokens: Tokens = vec![];
            let mut maker = Maker::new().string(loc());
            maker.push_str("Hello", &loc2()).unwrap();
            let result = maker.make_token(&loc2(), &mut tokens);

            assert_eq!(
                Err(TokenizerErr::UnclosedString {
                    string: "Hello".into(),
                    location: loc2()
                }),
                result
            );
            assert_eq!(
                Maker::String {
                    location: loc(),
                    string: "Hello".into()
                },
                maker
            );
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
            let mut maker = Maker::new().comment(Location::default());

            let expected = Maker::Comment {
                location: Location::default(),
                string: "c".into(),
            };
            let result = maker.push_char('c', &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn empty_returns_error() {
            let mut maker = Maker::new();

            let expected = Maker::new();
            let result = maker.push_char('c', &loc2());

            assert_eq!(
                Err(TokenizerErr::AddCharToUninitializedState {
                    char: 'c',
                    location: loc2()
                }),
                result
            );
            assert_eq!(expected, maker);
        }

        #[test]
        fn identifier_is_added_to() {
            let mut maker = Maker::new().identifier(loc());

            let expected = Maker::Identifier {
                location: loc(),
                string: "c".into(),
            };
            let result = maker.push_char('c', &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn string_is_added_to() {
            let mut maker = Maker::new().string(loc());

            let expected = Maker::String {
                location: loc(),
                string: "c".into(),
            };
            let result = maker.push_char('c', &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }
    }

    mod push_str {
        use super::*;

        #[test]
        fn comment_is_added_to() {
            let mut maker = Maker::new().comment(Location::default());

            let expected = Maker::Comment {
                location: Location::default(),
                string: "cab".into(),
            };
            let result = maker.push_str("cab", &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn empty_returns_error() {
            let mut maker = Maker::new();

            let expected = Maker::new();
            let result = maker.push_str("cab", &loc2());

            assert_eq!(
                Err(TokenizerErr::AddCharToUninitializedState {
                    char: 'c',
                    location: loc2()
                }),
                result
            );
            assert_eq!(expected, maker);
        }

        #[test]
        fn identifier_is_added_to() {
            let mut maker = Maker::new().identifier(loc());

            let expected = Maker::Identifier {
                location: loc(),
                string: "cab".into(),
            };
            let result = maker.push_str("cab", &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }

        #[test]
        fn string_is_added_to() {
            let mut maker = Maker::new().string(loc());

            let expected = Maker::String {
                location: loc(),
                string: "cab".into(),
            };
            let result = maker.push_str("cab", &loc2());

            assert_eq!(Ok(()), result);
            assert_eq!(expected, maker);
        }
    }

    mod string {
        use super::*;

        #[test]
        fn sets_proper_state() {
            let expected = Maker::String {
                location: loc(),
                string: String::default(),
            };
            assert_eq!(expected, Maker::new().string(loc()))
        }
    }
}
