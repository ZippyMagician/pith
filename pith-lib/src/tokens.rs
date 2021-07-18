#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Token {
    Equal,
    Star,
    Slash,
    ForwardSlash,
    Ampersand,
    UpArrow,
    DownArrow,
    LeftArrow,
    RightArrow,
    Colon,
    Period,
    Comma,
    Plus,
    Minus,
    AtSign,
    Percentage,
    Tilde,
    Pipe,
    Underscore,
    Pound,
    Exclamation,
    LeftBracket,
    RightBracket,
    DollarSign,
    Linefeed,
    Number(f64),

    Vector(f64, f64),
    Line(usize),
    Jmp(usize),
}

impl Token {
    pub fn into_value(self) -> f64 {
        match self {
            Self::Number(n) => n,
            _ => panic!("Could not convert {:?} to a Token::Number", self),
        }
    }
}
