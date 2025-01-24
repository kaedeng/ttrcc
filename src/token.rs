#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
    Identifier,
    Constant,
    IntKey,
    VoidKey,
    ReturnKey,
    OpenPar,
    ClosePar,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Unknown
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
    pub token: TokenType,
    pub val: String
}