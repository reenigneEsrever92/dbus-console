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

#[derive(Debug, PartialEq, Eq)]
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
            end: subject.len(),
            subject,
        }
    }

    fn slice(&self) -> Option<&'a str> {
        if self.start < self.end {
            Some(&self.subject[self.start..self.end])
        } else {
            None
        }
    }

    fn step(&mut self) {
        if self.start < self.end {
            self.end -= 1;
        }
    }

    fn next(&mut self) {
        self.start = self.end;
        self.end = self.subject.len();
    }
}

#[derive(Default, Debug, PartialEq, Eq, Clone)]
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
            content: tuple.0.slice().unwrap(),
            token_type: tuple.1,
        }
    }
}

#[derive(Debug)]
struct TokenizerError<'a> {
    message: &'a str,
    span: Span,
}

#[derive(Debug, PartialEq)]
enum TokenType {
    Number,
    String,
    StructStart, // (
    StructEnd,   // )
    ArrayStart,  // [
    ArrayEnd,    // ]
    DictStart,   // {
    DictEnd,     // }
    DictAssignmentOperator,
    Seperator,
    Whitespace,
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
    dict_assignment: Regex,
    seperator_regex: Regex,
    whitespace_regex: Regex,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            number_regex: Regex::new(r"^-?[0-9]+(\.[0-9]+)?$").unwrap(),
            string_regex: Regex::new(r#"^(("[a-zA-Z0-9_\.]*")|('[a-zA-Z0-9_\.]*'))$"#).unwrap(),
            struct_start_regex: Regex::new(r"^\($").unwrap(),
            struct_end_regex: Regex::new(r"^\)$").unwrap(),
            array_start_regex: Regex::new(r"^\[$").unwrap(),
            array_end_regex: Regex::new(r"^\]$").unwrap(),
            dict_start_regex: Regex::new(r"^\{$").unwrap(),
            dict_end_regex: Regex::new(r"^\}$").unwrap(),
            seperator_regex: Regex::new(r"^,$").unwrap(),
            whitespace_regex: Regex::new(r"^[ ]+$").unwrap(),
            dict_assignment: Regex::new(r"^:$").unwrap(),
        }
    }

    fn tokenize<'a>(&self, sub: &'a str) -> Result<TokenStream<'a, TokenType>, TokenizerError> {
        // TODO rework

        let mut cursor = Cursor::new(sub);
        let mut token_stream = TokenStream::default();

        while let Some(slice) = cursor.slice() {
            match self.match_token(slice) {
                Some(token) => {
                    token_stream.push((&cursor, token).into());
                    cursor.next();
                }
                None => cursor.step(),
            }
        }

        Ok(token_stream)
    }

    fn match_token(&self, slice: &str) -> Option<TokenType> {
        if self.struct_start_regex.is_match(slice) {
            Some(TokenType::StructStart)
        } else if self.struct_end_regex.is_match(slice) {
            Some(TokenType::StructEnd)
        } else if self.dict_start_regex.is_match(slice) {
            Some(TokenType::DictStart)
        } else if self.dict_end_regex.is_match(slice) {
            Some(TokenType::DictEnd)
        } else if self.dict_assignment.is_match(slice) {
            Some(TokenType::DictAssignmentOperator)
        } else if self.array_start_regex.is_match(slice) {
            Some(TokenType::ArrayStart)
        } else if self.array_end_regex.is_match(slice) {
            Some(TokenType::ArrayEnd)
        } else if self.string_regex.is_match(slice) {
            Some(TokenType::String)
        } else if self.number_regex.is_match(slice) {
            Some(TokenType::Number)
        } else if self.seperator_regex.is_match(slice) {
            Some(TokenType::Seperator)
        } else if self.whitespace_regex.is_match(slice) {
            Some(TokenType::Whitespace)
        } else {
            None
        }
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
                    token_type: TokenType::StructStart
                },
                Token {
                    span: super::Span { start: 1, end: 2 },
                    content: ")",
                    token_type: TokenType::StructEnd
                }
            ]),
            token_stream.unwrap()
        );
    }

    #[test]
    fn test_complex_structure() {
        let tokenizer = Tokenizer::new();
        let token_stream =
            tokenizer.tokenize("(\"test\", -3.1, { \"key\": \"value\" }, [ 1, 2, 3 ])");

        assert_eq!(
            token_stream.unwrap(),
            TokenStream::new(vec![
                Token {
                    span: super::Span { start: 0, end: 1 },
                    content: "(",
                    token_type: TokenType::StructStart
                },
                Token {
                    span: super::Span { start: 1, end: 7 },
                    content: "\"test\"",
                    token_type: TokenType::String
                },
                Token {
                    span: super::Span { start: 7, end: 8 },
                    content: ",",
                    token_type: TokenType::Seperator
                },
                Token {
                    span: super::Span { start: 8, end: 9 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 9, end: 13 },
                    content: "-3.1",
                    token_type: TokenType::Number
                },
                Token {
                    span: super::Span { start: 13, end: 14 },
                    content: ",",
                    token_type: TokenType::Seperator
                },
                Token {
                    span: super::Span { start: 14, end: 15 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 15, end: 16 },
                    content: "{",
                    token_type: TokenType::DictStart
                },
                Token {
                    span: super::Span { start: 16, end: 17 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 17, end: 22 },
                    content: "\"key\"",
                    token_type: TokenType::String
                },
                Token {
                    span: super::Span { start: 22, end: 23 },
                    content: ":",
                    token_type: TokenType::DictAssignmentOperator
                },
                Token {
                    span: super::Span { start: 23, end: 24 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 24, end: 31 },
                    content: "\"value\"",
                    token_type: TokenType::String
                },
                Token {
                    span: super::Span { start: 31, end: 32 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 32, end: 33 },
                    content: "}",
                    token_type: TokenType::DictEnd
                },
                Token {
                    span: super::Span { start: 33, end: 34 },
                    content: ",",
                    token_type: TokenType::Seperator
                },
                Token {
                    span: super::Span { start: 34, end: 35 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 35, end: 36 },
                    content: "[",
                    token_type: TokenType::ArrayStart
                },
                Token {
                    span: super::Span { start: 36, end: 37 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 37, end: 38 },
                    content: "1",
                    token_type: TokenType::Number
                },
                Token {
                    span: super::Span { start: 38, end: 39 },
                    content: ",",
                    token_type: TokenType::Seperator
                },
                Token {
                    span: super::Span { start: 39, end: 40 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 40, end: 41 },
                    content: "2",
                    token_type: TokenType::Number
                },
                Token {
                    span: super::Span { start: 41, end: 42 },
                    content: ",",
                    token_type: TokenType::Seperator
                },
                Token {
                    span: super::Span { start: 42, end: 43 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 43, end: 44 },
                    content: "3",
                    token_type: TokenType::Number
                },
                Token {
                    span: super::Span { start: 44, end: 45 },
                    content: " ",
                    token_type: TokenType::Whitespace
                },
                Token {
                    span: super::Span { start: 45, end: 46 },
                    content: "]",
                    token_type: TokenType::ArrayEnd
                },
                Token {
                    span: super::Span { start: 46, end: 47 },
                    content: ")",
                    token_type: TokenType::StructEnd
                },
            ]),
        );
    }
}
