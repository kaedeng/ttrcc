use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


use token::Token;
pub mod token;

#[cfg(test)]
#[path="./unit_tests/lexer_test.rs"]
mod lexer_test;

pub fn lex(file_path: &str) -> Vec<Token>{
    let tokens: Vec<Token> = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok){
            let mut characters = line.chars();

            let mut left: usize = 0;
            let mut right: usize = 0;

            let msg = "Failing parsing of line";

            while right <= left && left <= right {
                if !is_delimiter(characters.nth(right).expect(msg)) {
                    right += 1;
                }

                if is_delimiter(characters.nth(right).expect(msg)) && left == right {
                    if is_operator(characters.nth(right).expect(msg)) {
                        println!("Token: Operator, Value: {}", characters.nth(right).expect(msg))
                    }
                    right += 1;
                    left = right;
                }
                else if is_delimiter(characters.nth(right).expect(msg)) && left != right || right == characters.clone().count() && left != right {
                    let substr: String = characters.clone().skip(left).take(right - left).collect();
                    if is_keyword(&substr) {
                        println!("Token: Keyword, Value: {}", substr);
                    }
                    else if is_integer(&substr) {
                        println!("Token: Integer, Value: {}", substr);
                    }
                    else if is_valid_identifier(&substr) {
                        println!("Token: Identifier, Value: {}", substr);
                    }
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

fn is_integer(st : &str) -> bool{
    if st.is_empty() {
        return false;
    }
    let mut count: usize = 0;
    let mut ch = st.chars();
    while ch.nth(count).expect("Failed parsing of line").is_digit(10) {
        count += 1;
    }
    ch.nth(count).is_none()
}