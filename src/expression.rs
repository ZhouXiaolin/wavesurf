use std::fmt;

#[derive(Clone, Debug)]
pub enum Expression {
    Constant(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Root(Box<Expression>, Box<Expression>),
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
        Expression::power(expr, Expression::constant(-1.0))
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
        }
    }
}
