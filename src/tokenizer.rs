use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
struct TokenStream<'a, T> {
    tokens: Vec<Token<'a, T>>,
}

impl<'a, T> Default for TokenStream<'a, T> {
    fn default() -> Self {
        Self {
            tokens: Default::default(),
        }
    }
}

impl<'a, T> TokenStream<'a, T> {
    fn new(tokens: Vec<Token<'a, T>>) -> Self {
        Self { tokens }
    }

    fn push(&mut self, token: Token<'a, T>) {
        self.tokens.push(token);
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Token<'a, T> {
    span: Span,
    content: &'a str,
    token_type: T,
}

struct Cursor<'a> {
    start: usize,
    end: usize,
    subject: &'a str,
}

impl<'a> Cursor<'a> {
    fn new(subject: &'a str) -> Self {
        Self {
            start: 0,
            end: 1,
            subject,
        }
    }

    fn slice(&self) -> &'a str {
        &self.subject[self.start..self.end]
    }

    fn inc_end(&mut self) {
        self.end += 1;
    }

    fn inc_start(&mut self) {
        self.start += 1;
    }

    fn empty(&self) -> bool {
        self.end >= self.subject.len()
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Span {
    start: usize,
    end: usize,
}

impl<'a, T> From<(&Cursor<'a>, T)> for Token<'a, T> {
    fn from(tuple: (&Cursor<'a>, T)) -> Self {
        Self {
            span: Span {
                start: tuple.0.start,
                end: tuple.0.end,
            },
            content: tuple.0.slice(),
            token_type: tuple.1,
        }
    }
}

#[derive(Debug)]
struct TokenizerError<'a> {
    message: &'a str,
    span: Span,
}

#[derive(Debug)]
enum TokenType {
    Path,         // "org/dbus/introspect"
    Name,         // "org.dbus.introspect"
    FunctionName, // Introspect
    Literal,      // 4u32, "String",
    GroupStart,   // (
    GroupEnd,     // )
    SetStart,     // [
    SetEnd,       // ]
    DictStart,    // {
    DictEnd,      // }
}

struct Tokenizer {
    number_regex: Regex,
    string_regex: Regex,
    struct_start_regex: Regex,
    struct_end_regex: Regex,
    array_start_regex: Regex,
    array_end_regex: Regex,
    dict_start_regex: Regex,
    dict_end_regex: Regex,
    seperator: Regex,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            number_regex: Regex::new(r"^-?[0-9]+(\.[0-9]+)?$").unwrap(),
            string_regex: Regex::new(r"'.+'").unwrap(),
            struct_start_regex: Regex::new(r"\(").unwrap(),
            struct_end_regex: Regex::new(r"\)").unwrap(),
            array_start_regex: Regex::new(r"\]").unwrap(),
            array_end_regex: Regex::new(r"\[").unwrap(),
            dict_start_regex: Regex::new(r"\{").unwrap(),
            dict_end_regex: Regex::new(r"\}").unwrap(),
            seperator: Regex::new(r",[ ]*").unwrap(),
        }
    }

    fn tokenize<'a>(&self, sub: &'a str) -> Result<TokenStream<'a, TokenType>, TokenizerError> {
        let cursor = Cursor::new(sub);
        let mut token_stream = TokenStream::default();

        while !cursor.empty() {
            let mut last_token = None;
            match self.match_token(cursor.slice()) {
                Some(token) => last_token = Some(token),
                None => {
                    if let Some(token) = last_token {
                        token_stream.push((&cursor, token).into())
                    }
                }
            }
        }

        Ok(token_stream)
    }

    fn match_token(&self, slice: &str) -> Option<TokenType> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::{Token, TokenStream, TokenType, Tokenizer};

    #[test]
    fn test_path() {
        let tokenizer = Tokenizer::new();
        let token_stream = tokenizer.tokenize("()");

        assert_eq!(
            TokenStream::new(vec![
                Token {
                    span: super::Span { start: 0, end: 1 },
                    content: "(",
                    token_type: TokenType::GroupStart
                },
                Token {
                    span: super::Span { start: 1, end: 2 },
                    content: ")",
                    token_type: TokenType::GroupEnd
                }
            ]),
            token_stream.unwrap()
        );
    }
}
