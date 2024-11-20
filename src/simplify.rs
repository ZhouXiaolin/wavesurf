use crate::expression::Expression;

impl Expression {
    pub fn simplify(&self) -> Expression {
        match self {
            Expression::Constant(_) | Expression::Variable(_) => self.clone(),
            Expression::Add(left, right) => {
                let left = (**left).simplify();
                let right = (**right).simplify();
                match (&left, &right) {
                    // 0 + x = x
                    (Expression::Constant(c), _) if *c == 0.0 => right,
                    (_, Expression::Constant(c)) if *c == 0.0 => left,
                    // 常数合并
                    (Expression::Constant(c1), Expression::Constant(c2)) => {
                        Expression::constant(c1 + c2)
                    }
                    // 相同项合并
                    (Expression::Variable(v1), Expression::Variable(v2)) if v1 == v2 => {
                        Expression::multiply(
                            Expression::constant(2.0),
                            Expression::variable(v1)
                        )
                    }
                    _ => Expression::add(left, right),
                }
            }
            Expression::Subtract(left, right) => {
                let left = (**left).simplify();
                let right = (**right).simplify();
                match (&left, &right) {
                    // x - 0 = x
                    (_, Expression::Constant(c)) if *c == 0.0 => left,
                    // 常数合并
                    (Expression::Constant(c1), Expression::Constant(c2)) => {
                        Expression::constant(c1 - c2)
                    }
                    // x - x = 0
                    (Expression::Variable(v1), Expression::Variable(v2)) if v1 == v2 => {
                        Expression::constant(0.0)
                    }
                    _ => Expression::subtract(left, right),
                }
            }
            Expression::Multiply(left, right) => {
                let left = (**left).simplify();
                let right = (**right).simplify();
                match (&left, &right) {
                    // 0 * x = 0
                    (Expression::Constant(c), _) | (_, Expression::Constant(c)) if *c == 0.0 => {
                        Expression::constant(0.0)
                    }
                    // 1 * x = x
                    (Expression::Constant(c), _) if *c == 1.0 => right,
                    (_, Expression::Constant(c)) if *c == 1.0 => left,
                    // 常数合并
                    (Expression::Constant(c1), Expression::Constant(c2)) => {
                        Expression::constant(c1 * c2)
                    }
                    // 同类项合并
                    (Expression::Variable(v1), Expression::Variable(v2)) if v1 == v2 => {
                        Expression::power(
                            Expression::variable(v1),
                            Expression::constant(2.0)
                        )
                    }
                    _ => Expression::multiply(left, right),
                }
            }
            Expression::Divide(left, right) => {
                let left = (**left).simplify();
                let right = (**right).simplify();
                match (&left, &right) {
                    // 0 / x = 0
                    (Expression::Constant(c), _) if *c == 0.0 => Expression::constant(0.0),
                    // x / 1 = x
                    (_, Expression::Constant(c)) if *c == 1.0 => left,
                    // 常数合并
                    (Expression::Constant(c1), Expression::Constant(c2)) if *c2 != 0.0 => {
                        Expression::constant(c1 / c2)
                    }
                    // x / x = 1
                    (Expression::Variable(v1), Expression::Variable(v2)) if v1 == v2 => {
                        Expression::constant(1.0)
                    }
                    _ => Expression::divide(left, right),
                }
            }
            Expression::Power(base, exponent) => {
                let base = (**base).simplify();
                let exponent = (**exponent).simplify();
                match (&base, &exponent) {
                    // x^0 = 1
                    (_, Expression::Constant(c)) if *c == 0.0 => Expression::constant(1.0),
                    // x^1 = x
                    (_, Expression::Constant(c)) if *c == 1.0 => base,
                    // 0^n = 0 (n > 0)
                    (Expression::Constant(c), Expression::Constant(n)) if *c == 0.0 && *n > 0.0 => {
                        Expression::constant(0.0)
                    }
                    // 1^n = 1
                    (Expression::Constant(c), _) if *c == 1.0 => Expression::constant(1.0),
                    // 常数合并
                    (Expression::Constant(c), Expression::Constant(n)) => {
                        Expression::constant(c.powf(*n))
                    }
                    _ => Expression::power(base, exponent),
                }
            }
            Expression::Root(base, n) => {
                let base = (**base).simplify();
                let n = (**n).simplify();
                // 转换为幂函数处理
                Expression::power(
                    base,
                    Expression::divide(
                        Expression::constant(1.0),
                        n
                    )
                ).simplify()
            }
            Expression::Sin(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // sin(0) = 0
                        else { Expression::sin(simplified) }
                    }
                    _ => Expression::sin(simplified)
                }
            }
            Expression::Cos(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(1.0) }  // cos(0) = 1
                        else { Expression::cos(simplified) }
                    }
                    _ => Expression::cos(simplified)
                }
            }
            Expression::Tan(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // tan(0) = 0
                        else { Expression::tan(simplified) }
                    }
                    _ => Expression::tan(simplified)
                }
            }
            Expression::Arcsin(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // arcsin(0) = 0
                        else if x == 1.0 { Expression::constant(std::f64::consts::PI / 2.0) }  // arcsin(1) = π/2
                        else if x == -1.0 { Expression::constant(-std::f64::consts::PI / 2.0) }  // arcsin(-1) = -π/2
                        else { Expression::arcsin(simplified) }
                    }
                    _ => Expression::arcsin(simplified)
                }
            }
            Expression::Arccos(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 1.0 { Expression::constant(0.0) }  // arccos(1) = 0
                        else if x == -1.0 { Expression::constant(std::f64::consts::PI) }  // arccos(-1) = π
                        else if x == 0.0 { Expression::constant(std::f64::consts::PI / 2.0) }  // arccos(0) = π/2
                        else { Expression::arccos(simplified) }
                    }
                    _ => Expression::arccos(simplified)
                }
            }
            Expression::Arctan(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // arctan(0) = 0
                        else if x == 1.0 { Expression::constant(std::f64::consts::PI / 4.0) }  // arctan(1) = π/4
                        else if x == -1.0 { Expression::constant(-std::f64::consts::PI / 4.0) }  // arctan(-1) = -π/4
                        else { Expression::arctan(simplified) }
                    }
                    _ => Expression::arctan(simplified)
                }
            }
            Expression::Exp(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(1.0) }  // e^0 = 1
                        else if x == 1.0 { Expression::constant(std::f64::consts::E) }  // e^1 = e
                        else { Expression::exp(simplified) }
                    }
                    Expression::Ln(inner) => inner.simplify(),  // e^(ln(x)) = x
                    _ => Expression::exp(simplified)
                }
            }
            Expression::Ln(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 1.0 { Expression::constant(0.0) }  // ln(1) = 0
                        else if x == std::f64::consts::E { Expression::constant(1.0) }  // ln(e) = 1
                        else { Expression::ln(simplified) }
                    }
                    Expression::Exp(inner) => inner.simplify(),  // ln(e^x) = x
                    _ => Expression::ln(simplified)
                }
            }
            Expression::Log(base, expr) => {
                let simplified_base = base.simplify();
                let simplified_expr = expr.simplify();
                match (simplified_base, simplified_expr) {
                    (Expression::Constant(b), Expression::Constant(x)) => {
                        if x == 1.0 { Expression::constant(0.0) }  // log_b(1) = 0
                        else if x == b { Expression::constant(1.0) }  // log_b(b) = 1
                        else { Expression::log(Expression::constant(b), Expression::constant(x)) }
                    }
                    (a,b) => Expression::log(a, b)
                }
            }
            Expression::Sinh(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // sinh(0) = 0
                        else { Expression::sinh(simplified) }
                    }
                    _ => Expression::sinh(simplified)
                }
            }
            Expression::Cosh(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(1.0) }  // cosh(0) = 1
                        else { Expression::cosh(simplified) }
                    }
                    _ => Expression::cosh(simplified)
                }
            }
            Expression::Tanh(expr) => {
                let simplified = expr.simplify();
                match simplified {
                    Expression::Constant(x) => {
                        if x == 0.0 { Expression::constant(0.0) }  // tanh(0) = 0
                        else { Expression::tanh(simplified) }
                    }
                    _ => Expression::tanh(simplified)
                }
            }
        }
    }
}
