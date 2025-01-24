use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


use token::{Token, TokenType};
pub mod token;

#[cfg(test)]
#[path="./unit_tests/lexer_test.rs"]
mod lexer_test;
pub fn lex(file_path: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        // Iterate over all lines in the file
        for line in lines.map_while(Result::ok) {
            let characters = line.chars();
            let len = line.len();
            let mut left: usize = 0;
            let mut right: usize = 0;

            // Process each line until characters are consumed
            while right <= len && left <= right {
                if let Some(ch) = characters.clone().nth(right) {
                    if !is_delimiter(ch) {
                        right += 1;
                    }

                    if is_delimiter(ch) && left == right {
                        let mut token = Token {
                            token: TokenType::Unknown,
                            val: ch.to_string(),
                        };
                        if is_operator(ch) {
                            println!("Token: Operator, Value: {}", ch);
                        }
                        else{
                            match ch {
                                '(' => {token.token = TokenType::OpenPar;},
                                ')' => {token.token = TokenType::ClosePar;},
                                '{' => {token.token = TokenType::OpenBrace;},
                                '}' => {token.token = TokenType::CloseBrace;},
                                ';' => {token.token = TokenType::Semicolon;},
                                ' ' => (),
                                _ => {println!("Not implemented delimiter")},
                            }
                        }
                        right += 1;
                        left = right;
                    }

                    else if (is_delimiter(ch) && left != right)
                        || (right == len && left != right){
                        let substr: String = characters
                            .clone()
                            .skip(left)
                            .take(right - left)
                            .collect();

                        let mut token = Token {
                            token: TokenType::Unknown,
                            val: substr.clone(),
                        };

                        if is_keyword(&substr) {
                            match substr.as_str() {
                                "int" => token.token = TokenType::IntKey,
                                "return" => token.token = TokenType::ReturnKey,
                                "void" => token.token = TokenType::VoidKey,
                                _ => panic!("Token type {} not implemented yet!", substr),
                            }
                        }
                        
                        else if is_integer(&substr) {
                            token.token = TokenType::Constant; 
                        }
                        
                        else if is_valid_identifier(&substr)
                            && !is_delimiter(
                                characters
                                    .clone()
                                    .nth(right.saturating_sub(1))
                                    .expect(&format!("Failing parsing of line, l: {}, r: {}", left, right))){
                            token.token = TokenType::Identifier;
                        } else if !is_valid_identifier(&substr)
                            && !is_delimiter(
                                characters
                                    .clone()
                                    .nth(right.saturating_sub(1))
                                    .expect(&format!("Failing parsing of line, l: {}, r: {}", left, right))){
                            panic!("Unidentified Token: {}", substr);
                        }

                        tokens.push(token);
                        left = right;
                    }
                } else {
                    break;
                }
            }
        }
    }

    tokens
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_delimiter(ch: char) -> bool {
    ch == ' ' || ch == '+' || ch == '-'
    || ch == '*' || ch == '/' || ch == ','
    || ch == ';' || ch == '%' || ch == '>'
    || ch == '<' || ch == '=' || ch == '('
    || ch == ')' || ch == '[' || ch == ']'
    || ch == '{' || ch == '}'
}

fn is_operator(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*'
    || ch == '/' || ch == '>' || ch == '<'
    || ch == '='
}

fn is_valid_identifier(st : &str) -> bool {
    st.chars().nth(0) != Some('0') && st.chars().nth(0) != Some('1') && st.chars().nth(0) != Some('2')
    && st.chars().nth(0) != Some('3') && st.chars().nth(0) != Some('4')
    && st.chars().nth(0) != Some('5') && st.chars().nth(0) != Some('6')
    && st.chars().nth(0) != Some('7') && st.chars().nth(0) != Some('8')
    && st.chars().nth(0) != Some('9') && !is_delimiter(st.chars().nth(0).unwrap_or(' '))
}

fn is_keyword(st : &str) -> bool {
    let keywords = 
        vec![
            "auto",     "break",    "case",     "char",
            "const",    "continue", "default",  "do",
            "double",   "else",     "enum",     "extern",
            "float",    "for",      "goto",     "if",
            "int",      "long",     "register", "return",
            "short",    "signed",   "sizeof",   "static",
            "struct",   "switch",   "typedef",  "union",
            "unsigned", "void",     "volatile", "while"
            ];
    
    for key in keywords {
        if st.eq(key){
            return true;            
        }
    }
    false
    
}

fn is_integer(st: &str) -> bool {
    if st.is_empty() {
        return false;
    }
    let mut iter = st.chars();
    while let Some(c) = iter.next() {
        if !c.is_ascii_digit() {
            return false;
        }
    }
    true
}
