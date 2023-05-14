#[cfg(test)]
mod test {
    use crate::token::Token;
    use crate::lexer::Lexer;
    fn to_tokenizer_stream(code: &'static str) -> Vec::<Token> {
        let mut lexer = Lexer::new(String::from(code));
        let mut token_vec = Vec::<Token>::new();
        loop {
            let t = lexer.next_token();
            token_vec.push(t.clone());
            match t {
                Token::EOF => {
                    break;
                }
                _ => {
                    continue;
                }
            }
        }
        return token_vec;
    }
    fn is_equal(token_stream: Vec::<Token>, expect_stream: Vec::<Token>) {
        for i in 1..token_stream.len() {
            assert_eq!(token_stream[i], expect_stream[i]);
        }
    }
    fn expect(code: &'static str, expect_stream: Vec::<Token>) {
        let token_stream = to_tokenizer_stream(code);
        is_equal(token_stream, expect_stream)
    }
    /* ========================================================
         Tokenize Expression
      =========================================================
     */
    #[test]
    fn test_plus_tokenize() {
        expect("+", vec![Token::Plus, Token::EOF]);
    }
    #[test]
    #[should_panic]
    fn test_plus_assigment_should_failed() {
        expect("+=", vec![]);
    }
    #[test]
    #[should_panic]
    fn test_plusplus_update_operator_should_failed() {
        expect("++", vec![]);
    }
    /* ========================================================
         Tokenize Statement
      =========================================================
     */
    #[test]
    fn test_while_statement() {
        expect("
            while (n<=10) {
                n = n + 1;
            }
        ",
            vec![
                Token::WhileKeyword,
                Token::ParenthesesLeft,
                Token::Identifier(String::from("n")),
                Token::Lteq,
                Token::NumberLiteral(10.),
                Token::ParenthesesRight,
                Token::BracesLeft,
                Token::Identifier(String::from("n")),
                Token::Assign,
                Token::Identifier(String::from("n")),
                Token::Plus,
                Token::NumberLiteral(1.),
                Token::Semi,
                Token::BracesRight,
                Token::EOF,
            ]
        );
    }
}