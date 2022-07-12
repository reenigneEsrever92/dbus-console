use regex::Regex;

struct TokenStream<'a, T> {
    tokens: Vec<Token<'a, T>>,
}

struct Token<'a, T> {
    span: Span,
    content: &'a str,
    token_type: T,
}

#[derive(Debug, Clone)]
struct Span {
    start: u32,
    end: u32,
}

#[derive(Debug)]
struct TokenizerError<'a> {
    message: &'a str,
    span: Span,
}

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
    cursor: Cursor,
    i32_regex: Regex,
    u32_regex: Regex,
    string_regex: Regex,
    group_start_regex: Regex,
    group_end_regex: Regex,
    array_start_regex: Regex,
    array_end_regex: Regex,
    dict_start_regex: Regex,
    dict_end_regex: Regex,
}

struct Cursor {
    start: u32,
    end: u32,
}

impl Tokenizer {
    fn new() -> Self {
        Self {
            cursor: Cursor { start: 0, end: 0 },
            i32_regex: todo!(),
            u32_regex: todo!(),
            string_regex: todo!(),
            group_start_regex: todo!(),
            group_end_regex: todo!(),
            array_start_regex: todo!(),
            array_end_regex: todo!(),
            dict_start_regex: todo!(),
            dict_end_regex: todo!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Tokenizer;

    #[test]
    fn test_path() {
        let tokenizer = Tokenizer::new();
    }
}
