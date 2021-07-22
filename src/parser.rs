use crate::ast::{Infix, Query, Value};
use crate::lexer::Lexer;
use crate::parser::ParserError::*;
use crate::token::Token;
use log::debug;

type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub enum ParserError {
    ExpectedQueryToken(Token),
    ExpectedFilterToken(Token),
    ExpectedValueToken(Token),
    ExpectedSomethingToken(Token),
    ExpectedIdentifierToken(Token),
    ExpectedBooleanToken(Token),
    ExpectedIntegerToken(Token),
    ExpectedFloatToken(Token),
    ExpectedStringToken(Token),
    ExpectedLparen(Token),
    ExpectedRparen(Token),
    ExpectedComma(Token),
    ParseInt(String),
    ParseFloat(String),
    NotImplemented(String),
}

type ValueParseFn = fn(&mut Parser) -> Result<Value>;

pub struct Parser {
    lexer: Lexer,
    errors: Vec<ParserError>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new_from_string(s: String) -> Self {
        let lexer = Lexer::new(s);
        Self::new(lexer)
    }

    pub fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            errors: vec![],
            cur_token: Token::Illegal,
            peek_token: Token::Illegal,
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn input(&self) -> &str {
        self.lexer.input()
    }

    pub fn errors(&self) -> &[ParserError] {
        &self.errors
    }

    fn next_token(&mut self) {
        self.cur_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_query(&mut self) -> Result<Query> {
        match &self.cur_token {
            Token::And => return self.parse_and(),
            Token::Or => return self.parse_or(),
            _ => return self.parse_filter(),
        };
    }

    fn parse_and(&mut self) -> Result<Query> {
        self.expect_peek(Token::Lparen, ExpectedLparen)?;
        self.next_token();
        let mut queries: Vec<Query> = vec![];
        while self.cur_token != Token::Rparen {
            match self.parse_query() {
                Ok(q) => queries.push(q),
                Err(e) => return Err(e)
            }
            if self.cur_token == Token::Comma {
                self.next_token();
            }
            debug!("cur {}, {}", self.cur_token, self.peek_token);
        }
        self.next_token();
        return Ok(Query::And(queries));
    }

    fn parse_or(&mut self) -> Result<Query> {
        self.expect_peek(Token::Lparen, ExpectedLparen)?;
        self.next_token();
        let mut queries: Vec<Query> = vec![];
        while self.cur_token != Token::Rparen {
            match self.parse_query() {
                Ok(q) => queries.push(q),
                Err(e) => return Err(e)
            }
            if self.cur_token == Token::Comma {
                self.next_token();
            }
            debug!("cur {}, {}", self.cur_token, self.peek_token);
        }
        self.next_token();
        return Ok(Query::Or(queries));
    }

    fn parse_filter(&mut self) -> Result<Query> {
        // cur_token: eq, ne, ge, le, gt, lt
        let filter = match &self.cur_token {
            Token::Eq => Infix::Eq,
            Token::NotEq => Infix::NotEq,
            Token::Le => Infix::Le,
            Token::Ge => Infix::Ge,
            Token::Lt => Infix::Lt,
            Token::Gt => Infix::Gt,
            _ => return Err(ExpectedFilterToken(self.cur_token.clone())),
        };
        self.expect_peek(Token::Lparen, ExpectedLparen)?;

        self.next_token();
        let idnet = self.parse_identifier()?;
        self.expect_peek(Token::Comma, ExpectedComma)?;

        self.next_token();
        let value = self
            .parse_value()
            .ok_or_else(|| ExpectedValueToken(self.cur_token.clone()))?;
        let val = value(self)?;
        self.expect_peek(Token::Rparen, ExpectedRparen)?;
        self.next_token();
        Ok(Query::Filter(filter, idnet, val))
    }

    fn parse_value(&self) -> Option<ValueParseFn> {
        match &self.cur_token {
            Token::Ident(_) => Some(Parser::parse_identifier),
            Token::Int(_) => Some(Parser::parse_integer_literal),
            Token::Float(_) => Some(Parser::parse_float_literal),
            Token::Str(_) => Some(Parser::parse_string_literal),
            Token::True => Some(Parser::parse_boolean),
            Token::False => Some(Parser::parse_boolean),
            _ => None,
        }
    }

    fn parse_identifier(&mut self) -> Result<Value> {
        self.parse_identifier_string().map(Value::Identifier)
    }

    fn parse_identifier_string(&self) -> Result<String> {
        if let Token::Ident(ident) = &self.cur_token {
            Ok(ident.to_string())
        } else {
            Err(ExpectedIdentifierToken(self.cur_token.clone()))
        }
    }

    fn parse_integer_literal(&mut self) -> Result<Value> {
        if let Token::Int(int) = &self.cur_token {
            match int.parse() {
                Ok(value) => Ok(Value::IntegerLiteral(value)),
                Err(_) => Err(ParseInt(int.to_string())),
            }
        } else {
            Err(ExpectedIntegerToken(self.cur_token.clone()))
        }
    }

    fn parse_float_literal(&mut self) -> Result<Value> {
        if let Token::Float(float) = &self.cur_token {
            match float.parse() {
                Ok(value) => Ok(Value::FloatLiteral(value)),
                Err(_) => Err(ParseFloat(float.to_string())),
            }
        } else {
            Err(ExpectedFloatToken(self.cur_token.clone()))
        }
    }

    fn parse_string_literal(&mut self) -> Result<Value> {
        if let Token::Str(s) = &self.cur_token {
            Ok(Value::StringLiteral(s.to_string()))
        } else {
            Err(ExpectedStringToken(self.cur_token.clone()))
        }
    }
    fn parse_boolean(&mut self) -> Result<Value> {
        match &self.cur_token {
            Token::True => Ok(Value::Boolean(true)),
            Token::False => Ok(Value::Boolean(false)),
            _ => Err(ExpectedBooleanToken(self.cur_token.clone())),
        }
    }

    #[allow(dead_code)]
    // TODO: sort implementation will start after finished filter
    fn parse_sort(&mut self) -> Result<Query> {
        Err(NotImplemented("sort".to_string()))
    }

    fn expect_peek(&mut self, token: Token, expected: fn(Token) -> ParserError) -> Result<()> {
        if self.peek_token != token {
            return Err(expected(self.peek_token.clone()));
        }
        self.next_token();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Infix, Query, Value};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn single_filter() {
        let input = "eq(foo.bar,\"a\")";
        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);
        let query = parser.parse_query();
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            Query::Filter(
                Infix::Eq,
                Value::Identifier("foo.bar".to_string()),
                Value::StringLiteral("a".to_string()),
            )
        );
    }

    #[test]
    fn single_query() {
        let input = "and(eq(speed.max,100),lt(speed.min, 60.0))";
        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);
        let query = parser.parse_query();
        assert!(query.is_ok());

        assert_eq!(
            query.unwrap(),
            Query::And(
                vec![Query::Filter(
                    Infix::Eq,
                    Value::Identifier("speed.max".to_string()),
                    Value::IntegerLiteral(100),
                ),
                     Query::Filter(
                         Infix::Lt,
                         Value::Identifier("speed.min".to_string()),
                         Value::FloatLiteral(60.0),
                     )
                ],
            )
        );
    }

    #[test]
    fn nest_and_query() {
        let input = "and(and(eq(speed.max,100),lt(speed.min, 60.0)),eq(name,\"test\"))";
        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);
        let query = parser.parse_query();
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            Query::And(
                vec![Query::And(
                    vec![Query::Filter(
                        Infix::Eq,
                        Value::Identifier("speed.max".to_string()),
                        Value::IntegerLiteral(100),
                    ),
                         Query::Filter(
                             Infix::Lt,
                             Value::Identifier("speed.min".to_string()),
                             Value::FloatLiteral(60.0),
                         ),
                    ]
                ),
                     Query::Filter(
                         Infix::Eq,
                         Value::Identifier("name".to_string()),
                         Value::StringLiteral("test".to_string()),
                     )
                ],
            )
        );
    }

    #[test]
    fn nest_mixed_query() {
        let input = "and(or(eq(speed.max,100),lt(speed.min, 60.0)),eq(name,\"test\"))";
        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);
        let query = parser.parse_query();
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            Query::And(
                vec![Query::Or(
                    vec![
                        Query::Filter(
                            Infix::Eq,
                            Value::Identifier("speed.max".to_string()),
                            Value::IntegerLiteral(100),
                        ),
                        Query::Filter(
                            Infix::Lt,
                            Value::Identifier("speed.min".to_string()),
                            Value::FloatLiteral(60.0),
                        ),
                    ]
                ),
                     Query::Filter(
                         Infix::Eq,
                         Value::Identifier("name".to_string()),
                         Value::StringLiteral("test".to_string()),
                     )
                ]
            )
        );
    }

    #[test]
    fn nest_mixed_query2() {
        let input = "or(and(eq(foo,100),lt(bar, 60.0)),eq(baz,\"test\"))";
        let lexer = Lexer::new(input.to_owned());
        let mut parser = Parser::new(lexer);
        let query = parser.parse_query();
        assert!(query.is_ok());
        assert_eq!(
            query.unwrap(),
            Query::Or(
                vec![Query::And(
                    vec![Query::Filter(
                        Infix::Eq,
                        Value::Identifier("foo".to_string()),
                        Value::IntegerLiteral(100),
                    ),
                         Query::Filter(
                             Infix::Lt,
                             Value::Identifier("bar".to_string()),
                             Value::FloatLiteral(60.0),
                         )]
                ),
                     Query::Filter(
                         Infix::Eq,
                         Value::Identifier("baz".to_string()),
                         Value::StringLiteral("test".to_string()),
                     ),
                ]
            )
        );
    }
}