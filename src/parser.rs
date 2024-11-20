use crate::expression::Expression;
use std::iter::Peekable;
use std::str::Chars;

pub struct ExpressionParser<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(input: &'a str) -> Self {
        ExpressionParser {
            input: input.chars().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        self.parse_add_sub()
    }

    fn parse_add_sub(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_mul_div()?;

        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
                continue;
            }
            match c {
                '+' => {
                    self.input.next();
                    let right = self.parse_mul_div()?;
                    left = Expression::add(left, right);
                }
                '-' => {
                    self.input.next();
                    let right = self.parse_mul_div()?;
                    left = Expression::subtract(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul_div(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_power()?;

        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
                continue;
            }
            match c {
                '*' => {
                    self.input.next();
                    let right = self.parse_power()?;
                    left = Expression::multiply(left, right);
                }
                '/' => {
                    self.input.next();
                    let right = self.parse_power()?;
                    left = Expression::divide(left, right);
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expression, String> {
        let mut left = self.parse_primary()?;

        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
                continue;
            }
            if c == '^' {
                self.input.next();
                let right = self.parse_primary()?;
                left = Expression::power(left, right);
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<Expression, String> {
        self.skip_whitespace();
        match self.input.peek() {
            Some(&c) => {
                match c {
                    '0'..='9' => self.parse_number(),
                    '(' => {
                        self.input.next();
                        let expr = self.parse_expression()?;
                        self.expect_char(')')?;
                        Ok(expr)
                    }
                    'a'..='z' | 'A'..='Z' => self.parse_variable(),
                    _ => Err(format!("Unexpected character: {}", c)),
                }
            }
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_number(&mut self) -> Result<Expression, String> {
        let mut number = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        match number.parse::<f64>() {
            Ok(n) => Ok(Expression::constant(n)),
            Err(_) => Err(format!("Invalid number: {}", number)),
        }
    }

    fn parse_variable(&mut self) -> Result<Expression, String> {
        let mut name = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() {
                name.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        Ok(Expression::variable(&name))
    }

    fn expect_char(&mut self, expected: char) -> Result<(), String> {
        self.skip_whitespace();
        match self.input.next() {
            Some(c) if c == expected => Ok(()),
            Some(c) => Err(format!("Expected '{}', found '{}'", expected, c)),
            None => Err(format!("Expected '{}', found end of input", expected)),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&c) = self.input.peek() {
            if c.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.clone().next()
    }
}
