#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    Boolean,
    CloseBracket,
    CloseParen,
    Number,
    OpenBracket,
    OpenParen,
    String,
    Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenError {
    UnexpectedChar { span: Span, ch: char },
    UnterminatedString { span: Span },
    InvalidEscape { span: Span },
}

pub(crate) fn tokenize(input: &str) -> Result<Vec<Token>, TokenError> {
    let mut tokens = Vec::new();
    // TODO implement tokenizer logic here
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::{tokenize, Span, Token, TokenKind};

    #[test]
    fn parse_one_sexp_form() {
        let tokens = tokenize("(+ 2 40)").expect("tokenize");
        assert_eq!(
            tokens,
            vec![
                Token {
                    kind: TokenKind::OpenParen,
                    lexeme: "(".into(),
                    span: Span { start: 0, end: 1 }
                },
                Token {
                    kind: TokenKind::Symbol,
                    lexeme: "+".into(),
                    span: Span { start: 1, end: 2 }
                },
                Token {
                    kind: TokenKind::Number,
                    lexeme: "2".into(),
                    span: Span { start: 3, end: 4 }
                },
                Token {
                    kind: TokenKind::Number,
                    lexeme: "40".into(),
                    span: Span { start: 5, end: 7 }
                },
                Token {
                    kind: TokenKind::CloseParen,
                    lexeme: ")".into(),
                    span: Span { start: 7, end: 8 }
                },
            ]
        );
    }
}
