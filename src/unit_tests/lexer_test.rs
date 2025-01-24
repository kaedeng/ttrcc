use crate::driver::lexer::TokenType::*;
use crate::driver::lexer::Token;
#[allow(dead_code, unused_imports)]
mod lexer_test {
    use crate::driver::lexer::lex;

    use super::*;

    #[test]
    fn boiler_lexer(){
        assert_eq!(1, 1)
    }

    #[test]
    fn return_0(){
        let expected_tokens = vec![
            Token { token: IntKey, val: "int".to_string() },
            Token { token: Identifier, val: "main".to_string() },
            Token { token: ReturnKey, val: "return".to_string() },
            Token { token: Constant, val: "0".to_string() },
        ];
        assert_eq!(lex("./tests/return.c"), expected_tokens);
    }

}