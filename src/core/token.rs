pub enum Token
{
    // Identifiers
    IDENT(String),
    // Keywords
    IF,
    WHILE,
    LOOP,
    RETURN,
    // Operators
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    SET_EQUAL,
    TEST_EQUAL,
    // Bitwise

    // Constants
    CONST(String),  // multiple const data types with enum
    // Symbols
    LPAREN,
    RPAREN,
    LBRACK,
    RBRACK,
    NULL,
}