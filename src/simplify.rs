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
        }
    }
}
