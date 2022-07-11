trait TokenCap<'a> {
    fn span() -> Span;
    fn name() -> &'a str;
    fn content() -> &'a str;
}

struct TokenStream<'a> {
    tokens: Vec<Token<'a>>,
}

struct Token<'a> {
    span: Span,
    content: &'a str,
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

enum DBusToken {
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

fn parse<'a>(s: &'a str) -> Result<TokenStream<'a>, TokenizerError<'a>> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::parse;

    #[test]
    fn test_path() {
        let token_stream = parse("org/dbus/introspect").unwrap();
    }
}
