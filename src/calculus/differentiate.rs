use crate::Expression;

impl Expression {
    pub fn differentiate(&self, var: &str) -> Expression {
        match self {
            Expression::Constant(_) => Expression::constant(0.0),
            Expression::Variable(name) => {
                if name == var {
                    Expression::constant(1.0)
                } else {
                    Expression::constant(0.0)
                }
            }
            Expression::Add(left, right) => {
                Expression::add(left.differentiate(var), right.differentiate(var))
            }
            Expression::Subtract(left, right) => {
                Expression::subtract(left.differentiate(var), right.differentiate(var))
            }
            Expression::Multiply(left, right) => {
                // Product rule: d(u*v) = u*dv + v*du
                let du_v = Expression::multiply(left.differentiate(var), (**right).clone());
                let u_dv = Expression::multiply((**left).clone(), right.differentiate(var));
                Expression::add(du_v, u_dv)
            }
            Expression::Divide(left, right) => {
                // Quotient rule: d(u/v) = (v*du - u*dv)/(v^2)
                let v_du = Expression::multiply((**right).clone(), left.differentiate(var));
                let u_dv = Expression::multiply((**left).clone(), right.differentiate(var));
                let numerator = Expression::subtract(v_du, u_dv);
                let denominator = Expression::power((**right).clone(), Expression::constant(2.0));
                Expression::divide(numerator, denominator)
            }
            Expression::Power(base, exponent) => {
                match &**exponent {
                    Expression::Constant(n) => {
                        // Power rule: d(x^n) = n*x^(n-1)*dx
                        let new_power = Expression::power(
                            (**base).clone(),
                            Expression::constant(n - 1.0),
                        );
                        Expression::multiply(
                            Expression::constant(*n),
                            Expression::multiply(new_power, base.differentiate(var)),
                        )
                    }
                    _ => {
                        // General case using logarithmic differentiation
                        let ln_base = Expression::ln((**base).clone());
                        let derivative = Expression::multiply(
                            (**exponent).clone(),
                            Expression::multiply(ln_base, base.differentiate(var)),
                        );
                        derivative
                    }
                }
            }
            Expression::Root(base, n) => {
                // Convert root to power and differentiate
                let power = Expression::divide(Expression::constant(1.0), (**n).clone());
                Expression::power((**base).clone(), power).differentiate(var)
            }
            Expression::Sin(expr) => {
                // d/dx sin(u) = cos(u) * du/dx
                Expression::multiply(
                    Expression::cos((**expr).clone()),
                    expr.differentiate(var)
                )
            }
            Expression::Cos(expr) => {
                // d/dx cos(u) = -sin(u) * du/dx
                Expression::multiply(
                    Expression::multiply(
                        Expression::constant(-1.0),
                        Expression::sin((**expr).clone())
                    ),
                    expr.differentiate(var)
                )
            }
            Expression::Tan(expr) => {
                // d/dx tan(u) = sec²(u) * du/dx = (1 / cos²(u)) * du/dx
                Expression::multiply(
                    Expression::divide(
                        Expression::constant(1.0),
                        Expression::power(
                            Expression::cos((**expr).clone()),
                            Expression::constant(2.0)
                        )
                    ),
                    expr.differentiate(var)
                )
            }
            Expression::Arcsin(expr) => {
                // d/dx arcsin(x) = 1/sqrt(1 - x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::power(
                    Expression::subtract(one, Expression::power((**expr).clone(), two)),
                    Expression::constant(0.5)
                );
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Arccos(expr) => {
                // d/dx arccos(x) = -1/sqrt(1 - x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::power(
                    Expression::subtract(one, Expression::power((**expr).clone(), two)),
                    Expression::constant(0.5)
                );
                Expression::multiply(
                    inner_deriv,
                    Expression::multiply(
                        Expression::constant(-1.0),
                        Expression::divide(Expression::constant(1.0), denom)
                    )
                )
            }
            Expression::Arctan(expr) => {
                // d/dx arctan(x) = 1/(1 + x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::add(one, Expression::power((**expr).clone(), two));
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Exp(expr) => {
                // d/dx e^u = e^u * du/dx
                Expression::multiply(
                    Expression::exp((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Ln(expr) => {
                // d/dx ln(u) = 1/u * du/dx
                Expression::multiply(
                    Expression::divide(
                        Expression::constant(1.0),
                        (**expr).clone()
                    ),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Log(base, expr) => {
                // d/dx log_b(u) = 1/(u * ln(b))
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::multiply(
                    (**expr).clone(),
                    Expression::ln((**base).clone())
                );
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Sinh(expr) => {
                // d/dx sinh(u) = cosh(u) * du/dx
                Expression::multiply(
                    Expression::cosh((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Cosh(expr) => {
                // d/dx cosh(u) = sinh(u) * du/dx
                Expression::multiply(
                    Expression::sinh((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Tanh(expr) => {
                // d/dx tanh(u) = sech²(u) * du/dx = (1 - tanh²(u)) * du/dx
                Expression::multiply(
                    Expression::subtract(
                        Expression::constant(1.0),
                        Expression::power(
                            Expression::tanh((**expr).clone()),
                            Expression::constant(2.0)
                        )
                    ),
                    (**expr).clone().differentiate(var)
                )
            }
        }
    }
}