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
                // 检查是否是负幂，如果是，不要转换为除法
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
                    'a'..='z' | 'A'..='Z' => {
                        // 先尝试解析函数名
                        let name = self.parse_identifier()?;
                        match name.as_str() {
                            // 三角函数
                            "sin" => self.parse_function(Expression::sin),
                            "cos" => self.parse_function(Expression::cos),
                            "tan" => self.parse_function(Expression::tan),
                            // 反三角函数
                            "arcsin" => self.parse_function(Expression::arcsin),
                            "arccos" => self.parse_function(Expression::arccos),
                            "arctan" => self.parse_function(Expression::arctan),
                            // 自然对数和指数
                            "ln" => self.parse_function(Expression::ln),
                            "exp" => self.parse_function(Expression::exp),
                            "e" => {
                                // 检查是否后面跟着^，如果是则解析为自然指数
                                if let Some('^') = self.input.peek() {
                                    self.input.next(); // 消耗^
                                    let power = self.parse_primary()?;
                                    Ok(Expression::exp(power))
                                } else {
                                    Ok(Expression::constant(std::f64::consts::E))
                                }
                            }
                            // 双曲函数
                            "sinh" => self.parse_function(Expression::sinh),
                            "cosh" => self.parse_function(Expression::cosh),
                            "tanh" => self.parse_function(Expression::tanh),
                            // 如果不是函数名，就当作变量
                            _ => Ok(Expression::variable(&name))
                        }
                    }
                    _ => Err(format!("Unexpected character: {}", c)),
                }
            }
            None => Err("Unexpected end of input".to_string()),
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        let mut name = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_alphanumeric() {
                name.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        if name.is_empty() {
            Err("Expected identifier".to_string())
        } else {
            Ok(name)
        }
    }

    fn parse_function<F>(&mut self, constructor: F) -> Result<Expression, String>
    where
        F: FnOnce(Expression) -> Expression,
    {
        self.skip_whitespace();
        match self.input.peek() {
            Some('(') => {
                self.input.next(); // 消耗左括号
                let expr = self.parse_expression()?;
                self.expect_char(')')?;
                Ok(constructor(expr))
            }
            _ => Err("Expected '(' after function name".to_string())
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
