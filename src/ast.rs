use log::debug;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Query {
    And(Box<Query>, Box<Query>),
    Or(Box<Query>, Box<Query>),
    Sort(Prefix, Value),
    Filter(Infix, Value, Value),
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Identifier(String),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    Boolean(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Infix {
    Eq,
    NotEq,
    Le,
    Ge,
    Lt,
    Gt,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Prefix {
    Plus,
    Minus,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl Query {
    pub fn is_none(&self) -> bool {
        self == &Query::None
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::StringLiteral(s) => write!(f, "\"{}\"", s),
            Value::Identifier(s) => write!(f, "{}", s),
            Value::IntegerLiteral(i) => write!(f, "{}", i),
            Value::FloatLiteral(i) => write!(f, "{}", i),
            Value::Boolean(b) => write!(f, "{}", b),
        }
    }
}

// serde_json::Value個別の値との比較
impl Value {
    pub fn eq(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::StringLiteral(s) => {
                if let Some(v) = comparison.as_str() {
                    return v == s.as_str();
                }
            }
            // identiferは入ってこないが、一応文字列としても評価できるようにしておく
            Value::Identifier(s) => {
                if let Some(v) = comparison.as_str() {
                    return v == s.as_str();
                }
            }
            Value::IntegerLiteral(i) => {
                if let Some(v) = comparison.as_i64() {
                    return &v == i;
                }
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return &v == i;
                }
            }
            Value::Boolean(b) => {
                if let Some(v) = comparison.as_bool() {
                    return &v == b;
                }
            }
        }
        false
    }

    pub fn ne(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::StringLiteral(s) => {
                if let Some(v) = comparison.as_str() {
                    debug!("ne {}, {}", v, s);
                    return v != s.as_str();
                }
            }
            Value::Identifier(s) => {
                if let Some(v) = comparison.as_str() {
                    return v != s.as_str();
                }
            }
            Value::IntegerLiteral(i) => {
                if let Some(v) = comparison.as_i64() {
                    return &v != i;
                }
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return &v != i;
                }
            }
            Value::Boolean(b) => {
                if let Some(v) = comparison.as_bool() {
                    return &v != b;
                }
            }
        }
        false
    }

    pub fn lt(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::IntegerLiteral(i) => {
                if let Some(v) = comparison.as_i64() {
                    return i > &v;
                }
                false
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return i > &v;
                }
                false
            }
            // eq,ne以外の演算子が使えない
            _ => false,
        }
    }

    pub fn le(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::IntegerLiteral(i) => {
                if let Some(v) = comparison.as_i64() {
                    return i >= &v;
                }
                false
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return i >= &v;
                }
                false
            }
            // eq,ne以外の演算子が使えない
            _ => false,
        }
    }

    pub fn gt(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::IntegerLiteral(i) => {
                debug!("gt {} val {:?}", i, comparison);
                if let Some(v) = comparison.as_i64() {
                    return i < &v;
                }
                false
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return i < &v;
                }
                false
            }
            // eq,ne以外の演算子が使えない
            _ => false,
        }
    }

    pub fn ge(&self, comparison: serde_json::Value) -> bool {
        match self {
            Value::IntegerLiteral(i) => {
                if let Some(v) = comparison.as_i64() {
                    return i <= &v;
                }
                false
            }
            Value::FloatLiteral(i) => {
                if let Some(v) = comparison.as_f64() {
                    return i <= &v;
                }
                false
            }
            // eq,ne以外の演算子が使えない
            _ => false,
        }
    }
}

impl fmt::Display for Infix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Infix::NotEq => write!(f, "!="),
            Infix::Eq => write!(f, "="),
            Infix::Le => write!(f, "<="),
            Infix::Ge => write!(f, ">="),
            Infix::Lt => write!(f, "<"),
            Infix::Gt => write!(f, ">"),
        }
    }
}

impl fmt::Display for Prefix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
