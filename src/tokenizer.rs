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
            end: 0,
            subject,
        }
    }

    fn slice(&self) -> &'a str {
        &self.subject[self.start..self.end]
    }

    fn inc_end(&mut self) {
        self.end = if self.end < self.subject.len() {
            self.end + 1
        } else {
            self.end
        };
    }

    fn next(&mut self) {
        self.start = self.end;
    }

    fn peek(&self) -> Option<&str> {
        if self.end < self.subject.len() {
            Some(&self.subject[self.start..self.end + 1])
        } else {
            None
        }
    }

    fn can_increment(&self) -> bool {
        self.end < self.subject.len()
    }

    fn empty(&self) -> bool {
        self.end >= self.subject.len()
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

#[derive(Debug, PartialEq)]
enum TokenType {
    Number,      // Introspect
    String,      // 4u32, "String",
    StructStart, // (
    StructEnd,   // )
    ArrayStart,  // [
    ArrayEnd,    // ]
    DictStart,   // {
    DictEnd,     // }
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
            string_regex: Regex::new(r"^'.+'$").unwrap(),
            struct_start_regex: Regex::new(r"^\($").unwrap(),
            struct_end_regex: Regex::new(r"^\)$").unwrap(),
            array_start_regex: Regex::new(r"^\]$").unwrap(),
            array_end_regex: Regex::new(r"^\[$").unwrap(),
            dict_start_regex: Regex::new(r"^\{$").unwrap(),
            dict_end_regex: Regex::new(r"^\}$").unwrap(),
            seperator: Regex::new(r"^,[ ]*$").unwrap(),
        }
    }

    fn tokenize<'a>(&self, sub: &'a str) -> Result<TokenStream<'a, TokenType>, TokenizerError> {
        let mut cursor = Cursor::new(sub);
        let mut token_stream = TokenStream::default();

        while let Some(slice) = cursor.peek() {
            match self.match_token(slice) {
                Some(token) => {
                    cursor.inc_end();
                    if cursor.empty() {
                        token_stream.push((&cursor, token).into())
                    }
                }
                None => {
                    token_stream.push((&cursor, self.match_token(cursor.slice()).unwrap()).into());
                    cursor.next();
                }
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
        } else if self.array_start_regex.is_match(slice) {
            Some(TokenType::ArrayStart)
        } else if self.array_end_regex.is_match(slice) {
            Some(TokenType::ArrayEnd)
        } else if self.string_regex.is_match(slice) {
            Some(TokenType::String)
        } else if self.number_regex.is_match(slice) {
            Some(TokenType::Number)
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
}
