pub enum Value {
    String(String),
    Int(i32),
    Float(f32),
}

pub enum TokenKind {
    // literals
    Value(Value),  // values can be pushed and popped from stack
    Ident(String), // names of closures

    // Syntax
    LeftParen,  // begin closure
    RightParen, // end closure
    Semicolon,  // clears tape
    Pound,      // begins definition of named closure

    // Tape operators
    Left,  // move tape index to left
    Right, // move tape index to right
    //MLeft,
    //MRight, // pops from

    // Stack operators
    Add,    // pops 2 and pushes sum
    Pop,    // pops 1 and discards value
    Concat, // pops 2 strings and concatenates them
    Out,    // pops string and prints
    Fmt,    // formats string from popped values for every $ in string
    //Cons, // constructs list from two lists

    // Special
    EOF, // END OF FILE
}

pub struct Token {
    location: usize,
    pub kind: TokenKind,
}

pub fn lookup_keyword(ident: TokenKind) -> Option<TokenKind> {
    match ident {
        TokenKind::Ident(literal) => match literal.as_str() {
            "out" => Some(TokenKind::Out),
            "fmt" => Some(TokenKind::Fmt),
            _ => None,
        },
        _ => panic!(),
    }
}
