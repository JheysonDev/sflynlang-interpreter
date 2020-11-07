use crate::Position;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // General
    Identifier { value: String },
    Str { value: String },
    Num { value: f64 },

    True,
    False,

    // Keywords
    Let,
    Const,

    Func,
    Return,

    If,
    Else,

    // Data Types
    Boolean,
    String,
    Number,
    Void,

    // Signs
    Dot,
    Comma,
    Colon,
    Semicolon,

    Equal,
    DoubleEqual,

    Plus,
    PlusEqual,

    Minus,
    MinusEqual,

    Star,
    StarEqual,
    DoubleStar,
    DoubleStarEqual,

    Slash,
    SlashEqual,

    Percent,
    PercentEqual,

    LeftParentheses,
    RightParentheses,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    DoubleVBar,
    DoubleAmper,

    // Other
    EndOfLine,
    EndOfFile,
}

impl Token {
    /// Get the keyword if exists in the value or return it as an identifier.
    pub fn get_identifier_or_keyword(value: String) -> Token {
        match value.as_str() {
            // Keywords
            "let" => Token::Let,
            "const" => Token::Const,

            "func" => Token::Func,
            "return" => Token::Return,

            "if" => Token::If,
            "else" => Token::Else,

            // Data Types
            "boolean" => Token::Boolean,
            "string" => Token::String,
            "number" => Token::Number,
            "void" => Token::Void,

            // Identifier
            _ => Token::Identifier { value },
        }
    }

    /// Convert the token to a string.
    pub fn to_string(&self) -> String {
        match self {
            // General
            Self::Identifier { value } => value.to_string(),
            Self::Str { value: _ } => String::from("String"),
            Self::Num { value: _ } => String::from("Number"),

            Self::True => String::from("true"),
            Self::False => String::from("false"),

            // Keywords
            Self::Let => String::from("let"),
            Self::Const => String::from("const"),

            Self::Func => String::from("func"),
            Self::Return => String::from("return"),

            Self::If => String::from("if"),
            Self::Else => String::from("else"),

            // Data Types
            Self::Boolean => String::from("boolean"),
            Self::String => String::from("string"),
            Self::Number => String::from("number"),
            Self::Void => String::from("void"),

            // Signs
            Self::Dot => String::from("."),
            Self::Comma => String::from(","),
            Self::Colon => String::from(":"),
            Self::Semicolon => String::from(";"),

            Self::Equal => String::from("="),
            Self::DoubleEqual => String::from("=="),

            Self::Plus => String::from("+"),
            Self::PlusEqual => String::from("+="),

            Self::Minus => String::from("-"),
            Self::MinusEqual => String::from("-="),

            Self::Star => String::from("*"),
            Self::StarEqual => String::from("*="),
            Self::DoubleStar => String::from("**"),
            Self::DoubleStarEqual => String::from("**="),

            Self::Slash => String::from("/"),
            Self::SlashEqual => String::from("/="),

            Self::Percent => String::from("%"),
            Self::PercentEqual => String::from("%="),

            Self::LeftParentheses => String::from("("),
            Self::RightParentheses => String::from(")"),

            Self::LeftBrace => String::from("{"),
            Self::RightBrace => String::from("}"),

            Self::LeftBracket => String::from("["),
            Self::RightBracket => String::from("]"),

            Self::DoubleVBar => String::from("||"),
            Self::DoubleAmper => String::from("&&"),

            // Other
            Self::EndOfLine => String::from("\n"),
            Self::EndOfFile => String::from("<<EOF>>"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tok {
    /// Position object.
    position: Position,

    /// Token object.
    token: Token,
}

impl Tok {
    /// Create a new tok object using a position and a token.
    pub fn new(position: &Position, token: &Token) -> Self {
        Self {
            position: position.clone(),
            token: token.clone(),
        }
    }

    /// Get the position object of the tok.
    pub fn get_position(&self) -> Position {
        self.position.clone()
    }

    /// Get the token object of the tok.
    pub fn get_token(&self) -> Token {
        self.token.clone()
    }
}
