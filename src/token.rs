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
    Semicolon
}

pub struct Token {
    token: TokenType,
    val: String
}