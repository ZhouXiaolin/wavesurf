use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Constant(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Root(Box<Expression>, Box<Expression>),
    
    // 三角函数
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
    
    // 反三角函数
    Arcsin(Box<Expression>),
    Arccos(Box<Expression>),
    Arctan(Box<Expression>),
    
    // 指数和对数
    Exp(Box<Expression>),  // e^x
    Ln(Box<Expression>),   // 自然对数
    Log(Box<Expression>, Box<Expression>), // 任意底数的对数，第二个参数是底数
    
    // 双曲函数
    Sinh(Box<Expression>),
    Cosh(Box<Expression>),
    Tanh(Box<Expression>),
}

impl Expression {
    pub fn constant(value: f64) -> Self {
        Expression::Constant(value)
    }

    pub fn variable(name: &str) -> Self {
        Expression::Variable(name.to_string())
    }

    pub fn add(left: Expression, right: Expression) -> Self {
        Expression::Add(Box::new(left), Box::new(right))
    }

    pub fn subtract(left: Expression, right: Expression) -> Self {
        Expression::Subtract(Box::new(left), Box::new(right))
    }

    pub fn multiply(left: Expression, right: Expression) -> Self {
        Expression::Multiply(Box::new(left), Box::new(right))
    }

    pub fn divide(left: Expression, right: Expression) -> Self {
        Expression::Divide(Box::new(left), Box::new(right))
    }

    pub fn power(base: Expression, exponent: Expression) -> Self {
        Expression::Power(Box::new(base), Box::new(exponent))
    }

    pub fn root(base: Expression, n: Expression) -> Self {
        Expression::Root(Box::new(base), Box::new(n))
    }

    pub fn ln(expr: Expression) -> Expression {
        Expression::Ln(Box::new(expr))
    }

    pub fn sin(expr: Expression) -> Expression {
        Expression::Sin(Box::new(expr))
    }

    pub fn cos(expr: Expression) -> Expression {
        Expression::Cos(Box::new(expr))
    }

    pub fn tan(expr: Expression) -> Expression {
        Expression::Tan(Box::new(expr))
    }

    pub fn arcsin(expr: Expression) -> Expression {
        Expression::Arcsin(Box::new(expr))
    }

    pub fn arccos(expr: Expression) -> Expression {
        Expression::Arccos(Box::new(expr))
    }

    pub fn arctan(expr: Expression) -> Expression {
        Expression::Arctan(Box::new(expr))
    }

    pub fn exp(expr: Expression) -> Expression {
        Expression::Exp(Box::new(expr))
    }

    pub fn log(base: Expression, expr: Expression) -> Expression {
        Expression::Log(Box::new(base), Box::new(expr))
    }

    pub fn sinh(expr: Expression) -> Expression {
        Expression::Sinh(Box::new(expr))
    }

    pub fn cosh(expr: Expression) -> Expression {
        Expression::Cosh(Box::new(expr))
    }

    pub fn tanh(expr: Expression) -> Expression {
        Expression::Tanh(Box::new(expr))
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Constant(value) => {
                if value.fract() == 0.0 {
                    write!(f, "{}", *value as i64)
                } else {
                    write!(f, "{:.2}", value)
                }
            }
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Add(left, right) => {
                if let Expression::Constant(c) = **right {
                    if c < 0.0 {
                        write!(f, "{} - {}", left, -c)
                    } else {
                        write!(f, "{} + {}", left, right)
                    }
                } else {
                    write!(f, "{} + {}", left, right)
                }
            }
            Expression::Subtract(left, right) => {
                write!(f, "{} - {}", left, right)
            }
            Expression::Multiply(left, right) => {
                match (&**left, &**right) {
                    (Expression::Constant(c), expr) | (expr, Expression::Constant(c)) if *c == -1.0 => {
                        write!(f, "-{}", expr)
                    }
                    (Expression::Constant(c), expr) | (expr, Expression::Constant(c)) if *c == 1.0 => {
                        write!(f, "{}", expr)
                    }
                    _ => {
                        let need_parens_left = matches!(**left, Expression::Add(_, _) | Expression::Subtract(_, _));
                        let need_parens_right = matches!(**right, Expression::Add(_, _) | Expression::Subtract(_, _));
                        
                        if need_parens_left {
                            write!(f, "({}) * ", left)?;
                        } else {
                            write!(f, "{} * ", left)?;
                        }
                        
                        if need_parens_right {
                            write!(f, "({})", right)
                        } else {
                            write!(f, "{}", right)
                        }
                    }
                }
            }
            Expression::Divide(left, right) => {
                let need_parens_left = matches!(**left, Expression::Add(_, _) | Expression::Subtract(_, _));
                let need_parens_right = matches!(**right, Expression::Add(_, _) | Expression::Subtract(_, _));
                
                if need_parens_left {
                    write!(f, "({}) / ", left)?;
                } else {
                    write!(f, "{} / ", left)?;
                }
                
                if need_parens_right {
                    write!(f, "({})", right)
                } else {
                    write!(f, "{}", right)
                }
            }
            Expression::Power(base, exponent) => {
                let need_parens = matches!(**base, 
                    Expression::Add(_, _) | 
                    Expression::Subtract(_, _) | 
                    Expression::Multiply(_, _) | 
                    Expression::Divide(_, _)
                );
                
                if need_parens {
                    write!(f, "({})^{}", base, exponent)
                } else {
                    write!(f, "{}^{}", base, exponent)
                }
            }
            Expression::Root(base, n) => {
                write!(f, "√[{}]({})", n, base)
            }
            Expression::Sin(expr) => {
                write!(f, "sin({})", expr)
            }
            Expression::Cos(expr) => {
                write!(f, "cos({})", expr)
            }
            Expression::Tan(expr) => {
                write!(f, "tan({})", expr)
            }
            Expression::Arcsin(expr) => {
                write!(f, "arcsin({})", expr)
            }
            Expression::Arccos(expr) => {
                write!(f, "arccos({})", expr)
            }
            Expression::Arctan(expr) => {
                write!(f, "arctan({})", expr)
            }
            Expression::Exp(expr) => {
                write!(f, "exp({})", expr)
            }
            Expression::Ln(expr) => {
                write!(f, "ln({})", expr)
            }
            Expression::Log(base, expr) => {
                write!(f, "log[{}]({})", base, expr)
            }
            Expression::Sinh(expr) => {
                write!(f, "sinh({})", expr)
            }
            Expression::Cosh(expr) => {
                write!(f, "cosh({})", expr)
            }
            Expression::Tanh(expr) => {
                write!(f, "tanh({})", expr)
            }
        }
    }
}
