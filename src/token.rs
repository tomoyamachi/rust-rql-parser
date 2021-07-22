use std::fmt;
use Token::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident(String),  // eq, filter
    Int(String),    // 123456
    Float(String),  // 123.456
    Str(String), // "hello"
    True,
    False,

    // Query
    And,
    Or,
    Plus,
    Minus,
    Sort,
    Select,
    Values,
    Aggregate,
    Distinct,
    In,
    Out,
    Contains,
    Excludes,
    Limit,


    // Operators
    Eq,
    NotEq,
    Le,
    Ge,
    Lt,
    Gt,

    // Punct
    Comma,
    Lparen,
    Rparen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Illegal => write!(f, "ILLEGAL"),
            Eof => write!(f, "EOF"),

            Ident(ident) => write!(f, "{}", ident),
            Int(int) => write!(f, "{}", int),
            Float(float) => write!(f, "{}", float),
            // TODO: Escape `"` in a string as `\"`...
            Str(s) => write!(f, "\"{}\"", s),
            True => write!(f, "true"),
            False => write!(f, "false"),

            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
            And => write!(f, "and"),
            Or => write!(f, "or"),

            Eq => write!(f, "eq"),
            NotEq => write!(f, "ne"),
            Le => write!(f, "le"),
            Ge => write!(f, "ge"),
            Lt => write!(f, "lt"),
            Gt => write!(f, "gt"),

            Comma => write!(f, ","),
            Lparen => write!(f, "("),
            Rparen => write!(f, ")"),
            _ => write!(f, "not implemented"),
        }
    }
}

pub fn lookup_ident(ident: &str) -> Token {
    keyword_to_token(ident).unwrap_or_else(|| Ident(ident.to_owned()))
}

fn keyword_to_token(keyword: &str) -> Option<Token> {
    match keyword {
        "true" => Some(True),
        "false" => Some(False),
        "eq" => Some(Eq),
        "ne" => Some(NotEq),
        "le" => Some(Le),
        "ge" => Some(Ge),
        "lt" => Some(Lt),
        "gt" => Some(Gt),
        "and" => Some(And),
        "or" => Some(Or),
        // TODO : not implement
        "sort" => Some(Sort),
        "select" => Some(Select),
        "values" => Some(Values),
        "aggregate" => Some(Aggregate),
        "distinct" => Some(Distinct),
        "in" => Some(In),
        "out" => Some(Out),
        "contains" => Some(Contains),
        "excludes" => Some(Excludes),
        "limit" => Some(Limit),
        _ => None,
    }
}
