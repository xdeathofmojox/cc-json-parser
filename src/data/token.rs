#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Colon,
    SignPos,
    SignNeg,
    Digit(u8),
    FractionMarker,
    ExponentMarker,
    String(String),
    True,
    False,
    Null,
    Whitespace,
}
