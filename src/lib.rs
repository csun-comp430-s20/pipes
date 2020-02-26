mod token;

pub mod tokenizer {
    pub use crate::token::Token;

    pub fn tokenize(input: &str) -> Vec<Token> {
        vec![]
    }

    // helpers
    fn tokenize_int(word: &str) -> Option<Token> {
        None
    }
    fn tokenize_bool(word: &str) -> Option<Token> {
        None
    }
    fn tokenize_str(word: &str) -> Option<Token> {
        None
    }
    fn tokenize_var_or_keyword(word: &str) -> Option<Token> {
        None
    }

    // takes some iterator and modifies it
    fn skip_whitespace(iter: &mut str) {}
}

#[cfg(tests)]
pub mod tests {
    use self::tokenizer;

    #[test]
    fn dummy_test() {
        assert_eq!(true, true)
    }
}
